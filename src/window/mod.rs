mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::Application;
use glib::{clone, Object};
use gtk::ListItem;
use gtk::{gio, glib, NoSelection, SignalListItemFactory};

use crate::repo_object::RepoObject;
use crate::repo_row::RepoRow;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
            gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn repos(&self) -> gio::ListStore {
        self.imp()
            .repos
            .borrow()
            .clone()
            .expect("Error getting repos")
    }

    fn setup_repos(&self) {
        let model = gio::ListStore::new::<RepoObject>();

        self.imp().repos.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.repos()));
        self.imp().repo_list.set_model(Some(&selection_model));
    }

    fn get_repos(&self) {
        let repos = vec![
            ("morpheus".to_string(), "Edmonton".to_string()),
            ("neo".to_string(), "Saskatoon".to_string()),
        ];
        for repo in repos {
            let item = RepoObject::new(repo.0, repo.1);
            self.repos().append(&item);
        }
    }

    fn setup_callbacks(&self) {
        self.imp()
            .button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.get_repos();
            }));
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let repo_row = RepoRow::new();

            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be listitem")
                .set_child(Some(&repo_row));
        });

        factory.connect_bind(move |_, list_item| {
            let repo_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Need to be listitem")
                .item()
                .and_downcast::<RepoObject>()
                .expect("Needs to be repoobject");

            let repo_row = list_item
                .downcast_ref::<ListItem>()
                .expect("not listitem")
                .child()
                .and_downcast::<RepoRow>()
                .expect("Child needs to be reporow");

            repo_row.bind(&repo_object);
        });

        factory.connect_unbind(move |_, list_item| {
            // Get `TaskRow` from `ListItem`
            let repo_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<RepoRow>()
                .expect("The child has to be a `RepoRow`.");

            repo_row.unbind();
        });

        self.imp().repo_list.set_factory(Some(&factory));
    }
}
