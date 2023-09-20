use std::cell::Cell;
use gtk::glib;
use glib::Properties;
use adw::{prelude::*,subclass::prelude::*};

#[derive(Default,Properties)]
#[properties(wrapper_type=super::Myobject)]
pub struct Myobject{
    #[property(get,set)]
    pub number:Cell<i32>,
}
#[glib::object_subclass]
impl ObjectSubclass for Myobject {
    const NAME: &'static str = "MyObjectGtk";
    type Type = super::Myobject;
}
#[glib::derived_properties]
impl ObjectImpl for Myobject {}