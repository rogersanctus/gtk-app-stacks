use gtk::{
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
    glib::{self, subclass::InitializingObject}
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/supra-dev/gtk-app-stacks/ui/teste.ui")]
pub struct TesteView {
    #[template_child]
    pub label1: TemplateChild<gtk::Label>,
    #[template_child]
    pub label2: TemplateChild<gtk::Label>
}

#[glib::object_subclass]
impl ObjectSubclass for TesteView {
    const NAME: &'static str = "teste-view";
    type Type = super::TesteView;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TesteView {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for TesteView {}
impl BoxImpl for TesteView {}
