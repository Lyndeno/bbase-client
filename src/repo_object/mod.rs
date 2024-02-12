mod imp;

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
            .build();

        let imp = imp::RepoObject::from_object(&built);
        imp.data.borrow_mut().data = Some(repo);
        built
    }
}

#[derive(Default)]
pub struct RepoData {
    pub name: String,
    pub location: String,

    pub data: Option<RepoGetRepoList>,
}
