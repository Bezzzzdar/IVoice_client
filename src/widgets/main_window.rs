use gtk::prelude::*;
use gtk::{CssProvider, StyleContext};
use gtk::{Application, ApplicationWindow, Button, Box, Orientation, Image, Align};
use crate::widgets::icons::*;

fn load_css(path: &str) {
    let provider = CssProvider::new();

    if let Err(err) = provider.load_from_path(path) {
        eprintln!("Failed to load CSS from {}: {}", path, err);
        return;
    }

    if let Some(screen) = gtk::gdk::Screen::default() {
        StyleContext::add_provider_for_screen(&screen, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    } else {
        eprintln!("Failed to get default screen");
    }
}

pub struct ServerBar
{
    main_container: Box,
    server_container: Box,
    functional_container: Box,
    add_server_button: Button,
    server_list: Vec<Button>,
    settings_button: Button,
    direct_messages_button: Button,
}

impl ServerBar
{
    pub fn new() -> Self
    {
        /*Main container: Vertical Box for ServerBar*/
        let main_container = Box::new(Orientation::Vertical, 5);
        main_container.set_size_request(50, -1);
        main_container.set_vexpand(true);
        main_container.set_widget_name("side-bar-main-container");
        main_container.set_visible(true);
        
        /*Subcontainer: Vertical Box for list of servers*/
        let server_container = Box::new(Orientation::Vertical, 5);
        server_container.set_vexpand(true);
        server_container.set_valign(Align::Start);
        
        /*Subcontainer: Vertical Box for functional buttons*/
        let functional_container = Box::new(Orientation::Vertical, 5);
        functional_container.set_vexpand(true);
        functional_container.set_valign(Align::End);

        /*Buttons from list of servers*/
        let add_server_button = Button::builder().build();
        let pixbuf = load_image_from_file("icons\\add_server_white.png", 50, 50);
        let image = Image::from_pixbuf(Some(&pixbuf));
        add_server_button.set_image(Some(&image));
        add_server_button.set_always_show_image(true);
        add_server_button.set_widget_name("funtional-button");

        let server_list = Vec::new();

        /*Functional buttons*/
        let settings_button = Button::builder().build();
        settings_button.set_valign(Align::End);
        let pixbuf = load_image_from_file("icons\\settings_white.png", 50, 50);
        let image = Image::from_pixbuf(Some(&pixbuf));
        settings_button.set_image(Some(&image));
        settings_button.set_always_show_image(true);
        settings_button.set_widget_name("funtional-button");

        let direct_messages_button = Button::builder().build();
        direct_messages_button.set_valign(Align::End);
        let pixbuf = load_image_from_file("icons\\direct_messages_white.png", 50, 50);
        let image = Image::from_pixbuf(Some(&pixbuf));
        direct_messages_button.set_image(Some(&image));
        direct_messages_button.set_always_show_image(true);
        direct_messages_button.set_widget_name("funtional-button");
        
        /*Add buttons to subcontainers*/
        server_container.add(&add_server_button);

        functional_container.add(&direct_messages_button);
        functional_container.add(&settings_button);
        
        /*Add subcontainers to main container*/
        main_container.add(&server_container);
        main_container.add(&functional_container);
        
        /*Load css for all widgets*/
        load_css("src\\styles\\styles.css");
        
        let mut server_bar = ServerBar {
            main_container,
            server_container,
            functional_container,
            add_server_button,
            server_list,
            settings_button,
            direct_messages_button,
        };

        /*Add signals for pressed button*/
        server_bar.init_signals();

        server_bar
    } 

    fn init_signals(&mut self) {
        let server_container_clone = self.server_container.clone();
        // let server_list_clone = self.server_list.clone();
        self.add_server_button.connect_clicked(move |_| {
            let new_server_button = Button::builder().label("New").build();
            server_container_clone.add(&new_server_button);
            // server_list_clone.append(&new_server_button);
            new_server_button.show();
            println!("New server button added.");
        });

        self.settings_button.connect_clicked(|_| {
            println!("Settings button clicked.");
        });

        self.direct_messages_button.connect_clicked(|_| {
            println!("Direct Messages button clicked.");
        });
    }

    pub fn widget(&self) -> &gtk::Box
    {
        &self.main_container
    }
}

pub struct MainWindow {
    window: ApplicationWindow,
    server_bar: ServerBar,
    // functional_bar: Box,
    // main_widget: Box,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("IVoice")
            .default_width(1000)
            .default_height(700)
            .build();

        let global_frame = Box::new(Orientation::Horizontal, 0);

        let server_bar = ServerBar::new();
        
        global_frame.add(server_bar.widget());

        window.set_child(Some(&global_frame));
        window.show_all();

        MainWindow {
            window,
            server_bar,
        }
    }

    pub fn show(&self) {
        self.window.show();
    }
}
