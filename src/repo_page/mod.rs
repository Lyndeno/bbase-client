mod imp;

use adw::subclass::prelude::*;
use adw::{prelude::*, ActionRow};
use glib::Object;
use gtk::glib;

use crate::repo_object::RepoObject;

glib::wrapper! {
    pub struct RepoPage(ObjectSubclass<imp::RepoPage>)
    @extends adw::NavigationPage, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for RepoPage {
    fn default() -> Self {
        Self::new()
    }
}

impl RepoPage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_info(name: String, _location: String) -> Self {
        let obj: Self = Object::builder().property("title", &name).build();
        obj
    }

    pub fn bind(&self, repo_object: &RepoObject) {
        let imp = self.imp();
        let list = imp.prop_list.get();
        let mut bindings = imp.bindings.borrow_mut();

        let props = vec![
            ("Name", "name"),
            ("Region", "location"),
            ("Access Mode", "accessmode"),
            ("Last Modified", "lastmodified"),
            ("Current Usage", "currentusage"),
        ];

        for (title, prop) in props {
            let row = ActionRow::builder()
                .css_classes(["property"])
                .title(title)
                .build();
            let row_binding = repo_object
                .bind_property(prop, &row, "subtitle")
                .sync_create()
                .build();
            list.append(&row);
            bindings.push(row_binding);
        }

        let title_binding = repo_object
            .bind_property("name", self, "title")
            .sync_create()
            .build();
        bindings.push(title_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
