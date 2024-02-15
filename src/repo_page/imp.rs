use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::Binding;
use gtk::{glib, CheckButton, CompositeTemplate, Label};

// state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/lyndeno/bbase/repo_page.ui")]
pub struct RepoPage {
    #[template_child]
    pub name_label: TemplateChild<Label>,
    #[template_child]
    pub location_label: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for RepoPage {
    const NAME: &'static str = "RepoPage";
    type Type = super::RepoPage;
    type ParentType = adw::NavigationPage;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RepoPage {}
impl WidgetImpl for RepoPage {}
impl BoxImpl for RepoPage {}
impl NavigationPageImpl for RepoPage {}
