use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::glib;

use super::RepoData;

//State
#[derive(Properties, Default)]
#[properties(wrapper_type = super::RepoObject)]
pub struct RepoObject {
    #[property(name = "name", get, set, type = String, member = name)]
    #[property(name = "location", get, set, type = String, member = region)]
    #[property(name = "accessmode", get, set, type = String, member = access_mode)]
    #[property(name = "lastmodified", get, set, type = String, member = last_modified)]
    #[property(name = "currentusage", get, set, type = f64, member = current_usage)]
    pub data: RefCell<RepoData>,
}

#[glib::object_subclass]
impl ObjectSubclass for RepoObject {
    const NAME: &'static str = "RepoListObject";
    type Type = super::RepoObject;
}

#[glib::derived_properties]
impl ObjectImpl for RepoObject {}
