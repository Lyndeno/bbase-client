mod imp;

use adw::subclass::prelude::*;
use adw::Application;
use adw::{prelude::*, ActionRow};
use glib::{clone, Object};
use gtk::gio::ActionEntry;
use gtk::ListBox;
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
        self.imp().repo_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |obj| {
                let repo_object = obj.downcast_ref().expect("Obj should be RepoObject");
                let row = window.create_repo_row(repo_object);
                row.upcast()
            }),
        );

        self.set_repo_list_visible(&self.repos());
        self.repos()
            .connect_items_changed(clone!(@weak self as window => move |repos, _, _, _| {
                window.set_repo_list_visible(repos);
            }));
    }

    fn set_repo_list_visible(&self, repos: &gio::ListStore) {
        self.imp().repo_list.set_visible(repos.n_items() > 0);
    }

    fn get_repos(&self) {
        self.repos().remove_all();
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

    fn create_repo_row(&self, repo_object: &RepoObject) -> ActionRow {
        let row = ActionRow::builder().build();

        repo_object
            .bind_property("name", &row, "title")
            .bidirectional()
            .sync_create()
            .build();

        repo_object
            .bind_property("location", &row, "subtitle")
            .bidirectional()
            .sync_create()
            .build();

        row
    }

    fn setup_actions(&self) {
        let action_about = ActionEntry::builder("show_about")
            .activate(move |window, action, _| {
                let dialog = adw::AboutWindow::builder()
                    .application_name("BBase")
                    .developer_name("Lyndon Sanche")
                    .website("https://github.com/lyndeno/bbase-client")
                    .version(env!("CARGO_PKG_VERSION"))
                    .modal(true)
                    .developers(vec!["Lyndon Sanche <lsanche@lyndeno.ca>".to_string()])
                    .build();

                dialog.set_transient_for(Some(window));

                dialog.present();
            })
            .build();

        self.add_action_entries([action_about]);
    }
}
