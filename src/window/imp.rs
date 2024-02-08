use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};

// Object for state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/lyndeno/bbase/window.ui")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<Button>,
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

        self.button.connect_clicked(move |button| {
            button.set_label("Yo");
        });
    }
}

// Trait shared by al widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all app windows
impl ApplicationWindowImpl for Window {}
