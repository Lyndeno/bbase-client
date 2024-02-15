mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Object;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

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

    pub fn with_info(name: String, location: String) -> Self {
        let obj: Self = Object::builder().property("title", &name).build();
        obj.imp().location_label.set_label(&location);
        obj.imp().name_label.set_label(&name);
        obj
    }

    pub fn bind(&self, repo_object: &RepoObject) {
        let name_label = self.imp().name_label.get();
        let location_label = self.imp().location_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        let name_label_binding = repo_object
            .bind_property("name", &name_label, "label")
            .sync_create()
            .build();
        bindings.push(name_label_binding);

        let location_label_binding = repo_object
            .bind_property("location", &location_label, "label")
            .sync_create()
            .build();
        bindings.push(location_label_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
