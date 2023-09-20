mod window;
mod myobject;
use gtk::glib;
use adw::prelude::*;
use window::Window;
const APP_ID:&str="org.gtk_rs.Otro_gtk";
fn main()->glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app:&adw::Application) {
    let window=Window::new(app);
    window.present();
}
