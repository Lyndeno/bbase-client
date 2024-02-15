mod imp;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib::{self, subclass::types::FromObject, value::FromValue};

use crate::repos::repo_get::RepoGetRepoList;

use chrono::offset::Utc;
type DateTime = chrono::DateTime<Utc>;

glib::wrapper! {
    pub struct RepoObject(ObjectSubclass<imp::RepoObject>);
}

impl RepoObject {
    pub fn new(repo: RepoGetRepoList) -> Self {
        let r = RepoData::from(repo);
        let obj = Object::builder();
        let built = obj
            .property("name", r.name)
            .property("location", r.region)
            .property("accessmode", r.access_mode)
            .property("lastmodified", r.last_modified)
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

    pub last_modified: String,
}

impl From<RepoGetRepoList> for RepoData {
    fn from(v: RepoGetRepoList) -> Self {
        Self {
            name: v.name,
            region: v.region,
            access_mode: v.access_mode,
            last_modified: match v.last_modified {
                Some(x) => x.to_string(),
                None => "N/A".to_string(),
            },
        }
    }
}
