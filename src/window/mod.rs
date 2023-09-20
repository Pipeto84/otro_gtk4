mod imp;
use gtk::{gio,glib,SingleSelection};
use glib::{Object,clone};
use adw::{prelude::*,subclass::prelude::*,ActionRow};
use crate::myobject::Myobject;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow,gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
    fn elementos(&self)->gio::ListStore {
        self.imp()
            .elementos
            .borrow()
            .clone()
            .expect("no pudo obtener los elementos")
    }
    fn setup_model(&self) {
        let vector:Vec<Myobject>=(0..=20).map(Myobject::new).collect();
        let model=gio::ListStore::new::<Myobject>();
        model.extend_from_slice(&vector);
        self.imp().elementos.replace(Some(model));

        let selection_model=SingleSelection::new(Some(self.elementos()));
        self.imp().lista.bind_model(
            Some(&selection_model), 
            clone!(@weak self as window=>@default-panic,move|obj|{
                let myobject=obj.downcast_ref().expect("el objeto deberia ser Myobject");
                let row=window.create_list_row(myobject);
                row.upcast()
            }),  
        );
        self.set_list_visible(&self.elementos());
        self.elementos().connect_items_changed(clone!(@weak self as window=>move|elementos,_,_,_|{
            window.set_list_visible(elementos);
        }));
    }
    fn create_list_row(&self,myobject:&Myobject)->ActionRow {

        let row=ActionRow::builder()
            .activatable(true)
            .build();
        myobject
            .bind_property("number", &row, "title")
            .sync_create()
            .build();
        row.connect_activated(clone!(@weak myobject=>move|_|{
            myobject.aumentar();
        }));

        row
    }
    fn set_list_visible(&self,elementos:&gio::ListStore) {
        self.imp().lista.set_visible(elementos.n_items() > 0);
    }
    fn setup_actions(&self) {
        let action_x=gio::SimpleAction::new("pipe", None);
        action_x.connect_activate(move|_,_|{
            println!("hola pipe");
        });
        self.add_action(&action_x);
    }
}