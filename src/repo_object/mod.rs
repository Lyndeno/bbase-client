mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct RepoObject(ObjectSubclass<imp::RepoObject>);
}

impl RepoObject {
    pub fn new(name: String, location: String) -> Self {
        Object::builder()
            .property("name", name)
            .property("location", location)
            .build()
    }
}

#[derive(Default)]
pub struct RepoData {
    pub name: String,
    pub location: String,
}
