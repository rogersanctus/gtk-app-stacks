mod pages;

use gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
use gtk::gdk::Display;
use gtk::{
    gio,
    prelude::*,
    Application,
    ApplicationWindow,
    CssProvider,
    Stack,
    StyleContext
};

use pages::teste::TesteView;

fn main() {
    gio::resources_register_include!("compiled.res")
        .expect("Falha ao registrar recursos.");

        let app = Application::builder()
        .application_id("com.supra-dev.gtk-app-stacks")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let css_provider = CssProvider::new();
    css_provider.load_from_resource("/com/supra-dev/gtk-app-stacks/ui/main.css");
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to Display"),
        &css_provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION
    );

    let stack = Stack::builder()
        .css_name("main-stack")
        .build();

    let teste_view = TesteView::new();
    stack.add_named(&teste_view, Some("teste-page"));


    let home_view = gtk::Builder::from_resource("/com/supra-dev/gtk-app-stacks/ui/home.ui")
        .object::<gtk::Box>("home-view")
        .expect("Could not load view Home");

    stack.add_named(&gtk::Label::builder().label(&"A Label").build(), Some("label-page"));
    
    stack.add_named(&home_view, Some("home-page"));
    stack.set_visible_child_name("teste-page");

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&"Gtk App Stacks")
        .child(&stack)
        .build();
    window.present();
}
