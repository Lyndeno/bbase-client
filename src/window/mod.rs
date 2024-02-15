mod imp;

use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::RwLock;

use adw::subclass::prelude::*;
use adw::Application;
use adw::NavigationPage;
use adw::Toast;
use adw::{prelude::*, ActionRow};
use glib::{clone, Object};
use gtk::gio::ActionEntry;
use gtk::glib::property::PropertyGet;
use gtk::Button;
use gtk::ListBox;
use gtk::{gio, glib, NoSelection, SignalListItemFactory};

use crate::repo_object::RepoObject;
use crate::repo_page::RepoPage;
use crate::repo_row;
use crate::repo_row::RepoRow;
use crate::repos::get_repos;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
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

    async fn get_repos(&self) {}

    fn setup_callbacks(&self) {
        let (sender, receiver) = async_channel::bounded(1);
        self.imp()
            .refresh_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.imp().refresh_button.set_sensitive(false);
                window.imp().refresh_spinner.start();
                crate::runtime().spawn(clone!(@strong sender => async move {
                    let repos = get_repos().await;
                    sender.send(repos).await.expect("Channel is not open");
                }));
            }));

        glib::spawn_future_local(clone!(@weak self as window => async move {
            while let Ok(response) = receiver.recv().await {
                    window.repos().remove_all();
                    for repo in response {
                        let item = RepoObject::new(repo);
                        window.repos().append(&item);
                    }
                window.imp().refresh_button.set_sensitive(true);
                window.imp().refresh_spinner.stop();
                window.imp().mytoast.add_toast(Toast::new("Refreshed"));
            }
        }));
    }

    fn create_repo_row(&self, repo_object: &RepoObject) -> ActionRow {
        let button = Button::builder()
            .icon_name("arrow1-right-symbolic")
            .can_focus(false)
            .can_target(false)
            .has_frame(false)
            .build();
        let row = ActionRow::builder().activatable(true).build();

        row.add_suffix(&button);

        row.connect_activated(clone!(@weak self as window, @weak repo_object => move |_| {
            let page = RepoPage::new();
            page.bind(&repo_object);
            window.imp().repo_view.push(&page);
        }));

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
            .activate(move |window: &Self, action, _| {
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
