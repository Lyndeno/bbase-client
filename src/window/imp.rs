use std::cell::RefCell;
use std::sync::{Arc, RwLock};

use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::ListBox;
use gtk::{gio, glib, Button, CompositeTemplate};

// Object for state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/lyndeno/bbase/window.ui")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<Button>,

    #[template_child]
    pub repo_list: TemplateChild<ListBox>,
    pub repos: RefCell<Option<gio::ListStore>>,
}

// Trait for subclassing
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // Name needs to match class
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all gobjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        //self.button.connect_clicked(move |button| {
        //    button.set_label("Yo");
        //});

        let obj = self.obj();
        obj.setup_repos();
        obj.setup_callbacks();
        obj.setup_actions();
    }
}

// Trait shared by al widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all app windows
impl ApplicationWindowImpl for Window {}
