mod imp;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib::{self, subclass::types::FromObject, value::FromValue};

use crate::repos::repo_get::RepoGetRepoList;

glib::wrapper! {
    pub struct RepoObject(ObjectSubclass<imp::RepoObject>);
}

impl RepoObject {
    pub fn new(repo: RepoGetRepoList) -> Self {
        let obj = Object::builder();
        let built = obj
            .property("name", repo.name.clone())
            .property("location", repo.region.clone())
            .property("accessmode", repo.access_mode.clone())
            .build();

        built
    }

    pub fn data(&self) -> RepoData {
        self.imp().data.borrow().clone()
    }
}

#[derive(Default, Clone)]
pub struct RepoData {
    pub name: String,
    pub region: String,
    pub access_mode: String,
}
