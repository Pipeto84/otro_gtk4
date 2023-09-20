mod imp;
use gtk::glib;
use glib::Object;

glib::wrapper! {
    pub struct Myobject(ObjectSubclass<imp::Myobject>);
}
impl Myobject {
    pub fn new(number:i32)->Self {
        Object::builder().property("number", number).build()
    }
    pub fn aumentar(self) {
        self.set_number(self.number()*2);
    }
}