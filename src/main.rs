extern crate gtk;
extern crate gio;
extern crate gdk;

extern crate libhandy;
// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;
use libhandy::prelude::*;

use std::env;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};
fn switch_visible_child(){

}


fn main() {
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);

        // Then we set its size and a title.
        win.set_default_size(320, 200);
        win.set_title("Basic example");

        let leaf = libhandy::Leaflet::new();

        let page1 = gtk::Box::new(gtk::Orientation::Vertical,8);
        let page2 = gtk::Box::new(gtk::Orientation::Vertical,8);
        page2.set_size_request(600,600);

        let button = gtk::Label::new(Some("[   Drag PDF file here   ]"));
        button.set_size_request(600,400);
        let targets = vec![
            gtk::TargetEntry::new("STRING", gtk::TargetFlags::OTHER_APP, 0),
            gtk::TargetEntry::new("text/plain", gtk::TargetFlags::OTHER_APP, 0),
        ];
        button.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);

        let leaf_switcher = leaf.clone();
        let page_2_switcher = page2.clone();
        let spinner = gtk::Spinner::new();

        button.connect_drag_data_received(move |w, _, _, _, s, _, _|  {
            let mut value: &str = &s.get_text().unwrap();
            value = value.trim();
            let new_val = value.replace("file://","");
            println!("final: {}",new_val);
            match value.split('.').last().unwrap() {
                "pdf"=>{
                    w.set_text(value);
                    page_2_switcher.pack_start(&spinner, true,true,16);
                    spinner.start();
                    spinner.set_visible(true);
                    leaf_switcher.set_visible_child(&page_2_switcher);
                    let c = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("cp {} /tmp/",new_val))
                    .output()
                    .expect("much fail");
                    let actual = c.stdout;
                    println!("yeee: {}",actual.iter().map(|i|*i as char).collect::<String>().trim());
               },
                _ => {
                    gtk::MessageDialog::new(None::<&Window>,
                       DialogFlags::empty(),
                       MessageType::Info,
                       ButtonsType::Ok,
                       "pls insert pdf file").run();
                },
            }
        });



        let label2 = gtk::Label::new(Some("Invisible label on the right"));

        page1.pack_start(&button,true,true,8);
       // page2.pack_start(&label2,true,true,9);

        leaf.add(&page1);
        leaf.add(&page2);



        win.add(&leaf);
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
