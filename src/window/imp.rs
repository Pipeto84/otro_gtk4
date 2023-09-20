use std::cell::RefCell;
use gtk::{glib,gio,CompositeTemplate, ListBox};
use glib::subclass::InitializingObject;
use adw::subclass::prelude::*;

#[derive(CompositeTemplate,Default)]
#[template(file="window.ui")]
pub struct Window{
    #[template_child]
    pub lista:TemplateChild<ListBox>,
    pub elementos:RefCell<Option<gio::ListStore>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MiOtroGtk";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(_klass: &mut Self::Class) {
        _klass.bind_template();
    }
    fn instance_init(_obj: &InitializingObject<Self>) {
        _obj.init_template();
    }
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        let obj=self.obj();
        obj.setup_model();
        obj.setup_actions();
    }
}
impl ApplicationWindowImpl for Window {}
impl WindowImpl for Window {}
impl WidgetImpl for Window {}
impl AdwApplicationWindowImpl for Window {}