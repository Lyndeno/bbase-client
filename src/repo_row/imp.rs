use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::Binding;
use gtk::{glib, CompositeTemplate, Label};

// state
#[derive(Default, CompositeTemplate)]
#[template(file = "res/repo_row.blp")]
pub struct RepoRow {
    #[template_child]
    pub name_label: TemplateChild<Label>,
    #[template_child]
    pub location_label: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for RepoRow {
    const NAME: &'static str = "RepoListRow";
    type Type = super::RepoRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RepoRow {}
impl WidgetImpl for RepoRow {}
impl BoxImpl for RepoRow {}
