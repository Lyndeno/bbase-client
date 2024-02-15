mod imp;

use adw::subclass::prelude::*;
use adw::{prelude::*, ActionRow};
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
        obj
    }

    pub fn bind(&self, repo_object: &RepoObject) {
        let imp = self.imp();
        let list = imp.prop_list.get();
        let mut bindings = imp.bindings.borrow_mut();

        list.append(&property_row("Name", repo_object.name()));
        list.append(&property_row("Region", repo_object.location()));

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

fn property_row<A: ToString, B: ToString>(title: A, subtitle: B) -> ActionRow {
    ActionRow::builder()
        .title(title.to_string())
        .subtitle(subtitle.to_string())
        .css_classes(["property"])
        .build()
}
