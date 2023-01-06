#![allow(dead_code)]
use ansi_term::{Color::Black, Color::RGB};
use anyhow::{anyhow, Error, Result};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::blocking::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.gql",
    query_path = "src/graphql/query.gql",
    response_derives = "Debug"
)]
struct Query;

type Date = String;

#[derive(Debug)]
pub struct Contribution {
    pub date: String,
    pub count: i64,
    pub color: String,
}

const ENDPOINT: &str = "https://api.github.com/graphql";

pub fn post_query(user_name: String) -> Result<query::QueryUser, Error> {
    dotenv::dotenv().ok();
    let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");

    let vars = query::Variables { user_name };

    let client = Client::builder()
        .user_agent("grapql-rust/0.11.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let response_body = post_graphql::<Query, _>(&client, ENDPOINT, vars)?;

    let user_data = match response_body.data {
        Some(data) => data.user.ok_or(anyhow!("user not found"))?,
        None => return Err(anyhow!("unable to retrieve data")),
    };

    Ok(user_data)
}

pub fn parse_contributions(response: query::QueryUser) -> Vec<Contribution> {
    response
        .contributions_collection
        .contribution_calendar
        .weeks
        .into_iter()
        .flat_map(|week| week.contribution_days)
        .map(|day| Contribution {
            date: day.date,
            count: day.contribution_count,
            color: day.color,
        })
        .collect()
}

pub fn get_total_contributions(contributions: &Vec<Contribution>) -> i64 {
    contributions.iter().map(|c| c.count).sum()
}

pub fn print_contributions(contributions: Vec<Contribution>) {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let weeks = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    // convert hex color to rgb
    let rgb_colors: Vec<(u8, u8, u8)> = {
        let mut colors = Vec::new();
        for contribution in &contributions {
            let hex_str = u32::from_str_radix(&contribution.color[1..], 16).unwrap();
            let r = ((hex_str >> 16) & 0xFF) as u8;
            let g = ((hex_str >> 8) & 0xFF) as u8;
            let b = (hex_str & 0xFF) as u8;
            colors.push((r, g, b));
        }
        colors
    };

    println!(" {} {}", " ".repeat(5), months.join("\t"));
    for (i, week) in weeks.iter().enumerate() {
        print!("{} ", week);
        for j in 0..contributions.len() {
            if j % 7 == i {
                let color = RGB(rgb_colors[j].0, rgb_colors[j].1, rgb_colors[j].2);
                match contributions[j].count {
                    0 => print!("{} ", Black.paint("■")),
                    _ => print!("{} ", color.paint("■")),
                }
            }
        }
        println!();
    }
}
