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
    #[property(name = "location", get, set, type = String, member = location)]
    pub data: RefCell<RepoData>,
}

#[glib::object_subclass]
impl ObjectSubclass for RepoObject {
    const NAME: &'static str = "RepoListObject";
    type Type = super::RepoObject;
}

#[glib::derived_properties]
impl ObjectImpl for RepoObject {}
