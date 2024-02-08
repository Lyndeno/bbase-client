mod repo_object;
mod repo_row;
mod window;
use chrono::offset::Utc;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery, Response};
use gtk::ApplicationWindow;
use std::error::Error;
use window::Window;

use adw::Application;
use gtk::{gio, glib};
use gtk::{prelude::*, Button};

const APP_ID: &str = "org.lyndeno.bbase";

type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "query.graphql",
    response_derives = "Debug"
)]
struct RepoGet;

fn main() -> glib::ExitCode {
    println!("Hello, world!");

    /*
    let borg_token = std::env::var("BORG_TOKEN").expect("No token provided");

    let client = reqwest::blocking::Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", borg_token)).unwrap(),
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
    .unwrap();

    println!("{:?}", response_body);

    let response_data: repo_get::ResponseData = response_body.data.expect("Oops");

    let repos: Vec<String> = response_data
        .repo_list
        .unwrap()
        .iter()
        .flatten()
        .map(|test| test.name.clone())
        .collect();
    */

    // Start graphical app
    gio::resources_register_include!("mainwindow.gresource").expect("Failed to get resource");

    // create app
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);

    window.present();
}
