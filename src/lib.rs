use anyhow::{anyhow, Error, Result};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::blocking::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.gql",
    query_path = "src/query.gql",
    response_derives = "Debug"
)]
struct Query;

type Date = String;

struct Contribution {
    date: String,
    color: String,
}

const ENDPOINT: &str = "https://api.github.com/graphql";

pub fn post_query(user_name: String) -> Result<query::ResponseData, Error> {
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

    response_body
        .data
        .ok_or_else(|| anyhow!("no data in response"))
}


pub fn parse_contributions(response: query::ResponseData) -> Vec<Contribution> {
    let mut contributions = Vec::new();
    for week in response.user.contributions_collection.contribution_calendar.weeks {
        for day in week.contribution_days {
            contributions.push(Contribution {
                date: day.date.to_string(),
                color: day.color.to_string(),
            });
        }
    }
    contributions
}
