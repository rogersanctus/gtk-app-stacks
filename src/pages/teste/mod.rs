mod imp;

use gtk::{glib::{self, Object}};

glib::wrapper! {
    pub struct TesteView(ObjectSubclass<imp::TesteView>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget;
}

impl TesteView {
    pub fn new() -> Self {
        Object::new(&[]).expect("Could not create View Teste")
    }
}