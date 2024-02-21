use graphql_client::{reqwest::post_graphql, GraphQLQuery};

use chrono::offset::Utc;

use crate::repos::repo_get::RepoGetRepoList;
type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "query.graphql",
    response_derives = "Debug, Clone"
)]
pub struct RepoGet;

pub async fn get_repos() -> Vec<RepoGetRepoList> {
    let borg_token = std::env::var("BORG_TOKEN");

    if let Ok(t) = borg_token {
        let client = reqwest::Client::builder()
            .user_agent("graphql-rust/0.10.0")
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", t)).unwrap(),
                ))
                .collect(),
            )
            .build()
            .unwrap();

        let response_body = post_graphql::<RepoGet, _>(
            &client,
            "https://api.borgbase.com/graphql",
            repo_get::Variables,
        )
        .await
        .unwrap();

        let response_data: repo_get::ResponseData = response_body.data.expect("Oops");

        let repos: Vec<RepoGetRepoList> = response_data
            .repo_list
            .unwrap()
            .into_iter()
            .flatten()
            .collect();
        repos
    } else {
        Vec::default()
    }
}
