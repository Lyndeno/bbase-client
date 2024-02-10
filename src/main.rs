mod repo_object;
mod repo_row;
mod repos;
mod window;
use gtk::ApplicationWindow;
use std::error::Error;
use window::Window;

use adw::prelude::*;
use adw::Application;
use gtk::Button;
use gtk::{gio, glib};

const APP_ID: &str = "org.lyndeno.bbase";

#[async_std::main]
async fn main() -> glib::ExitCode {
    println!("Hello, world!");

    /*
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
