use anyhow::Result;
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

const ENDPOINT: &str = "https://api.github.com/graphql";

pub fn post_query(user_name: String) -> Result<(), anyhow::Error> {
    let github_token = "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";

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

    let response_data: query::ResponseData = response_body.data.expect("missing response data");

    println!("{:#?}", response_data.user);

    Ok(())
}
