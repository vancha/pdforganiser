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
        win.set_default_size(320, 200);//basic example stuff
        win.set_title("Basic example");


        //a leaflet that's going to hide a frame on the right that will later display the individual subpages of a dragged pdf
        let leaf = libhandy::Leaflet::new();

        //page1, the drag area
        let page1 = gtk::Box::new(gtk::Orientation::Vertical,8);
        //page2, contains the hidden frame
        let page2 = gtk::Box::new(gtk::Orientation::Vertical,8);
        //make page2 wider, so that the leaflet will automatically hide it when the program starts
        page2.set_size_request(600,600);

        //this label is the drag destination.. stupid,but it works
        let lab = gtk::Label::new(Some("[   Drag PDF file here   ]"));
        //make it chonkier
        lab.set_size_request(600,400);
        //which targets are allowed to drag stuff into our label? (only other apps)
        let targets = vec![
            gtk::TargetEntry::new("STRING", gtk::TargetFlags::OTHER_APP, 0),
            gtk::TargetEntry::new("text/plain", gtk::TargetFlags::OTHER_APP, 0),
        ];
        //what to do on drag?
        lab.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);

        //clone some widgets so that they can be used in the closure below
        let leaf_switcher = leaf.clone();
        let page_2_switcher = page2.clone();
        let spinner = gtk::Spinner::new();

        //closure that will execute once things have been dragged to the label
        lab.connect_drag_data_received(move |w, _, _, _, s, _, _|  { //all arguments disregarded, we only use w and s. (widget and source)
            let mut value: &str = &s.get_text().unwrap();
            value = value.trim();//remove the /n/r from linux file names
            let new_val = value.replace("file://","");//remove the starting file:// from linux file names
            println!("final: {}",new_val);//this is what my debugging looks like :P
            match value.split('.').last().unwrap() {//what's the file type?
                "pdf"=>{//if it's pdf
                    w.set_text(value);
                    page_2_switcher.pack_start(&spinner, true,true,16);//add spinner to second child
                    spinner.start();//start loading anymation on spinner
                    spinner.set_visible(true);
                    leaf_switcher.set_visible_child(&page_2_switcher);//set leaflet to show second child
                    let c = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("cp {} /tmp/",new_val))
                    .output()
                    .expect("much fail");//this fails now, before doing anything
                    let actual = c.stdout;//this is a collection of bytes i think
                    println!("yeee: {}",actual //so when printing it
                        .iter()//i need to iterate over the bytes
                        .map(|i|*i as char)//cast them to chars
                        .collect::<String>()//and make a string out of it
                        .trim()//removing the stupid trailing whitespace is just convenient
                    );
               },
                _ => {//if it's anything but pdf
                    gtk::MessageDialog::new(None::<&Window>,
                       DialogFlags::empty(),
                       MessageType::Info,
                       ButtonsType::Ok,
                       "pls insert pdf file").run();//popup says no
                },
            }
        });



        page1.pack_start(&lab,true,true,8);

        leaf.add(&page1);
        leaf.add(&page2);



        win.add(&leaf);
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
