use gtk::gio;

fn main() {
    gio::compile_resources(
        "resources",
        "resources/resource.gresource.xml",
        "compiled.res"
    );
}
