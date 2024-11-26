use gtk::{prelude::*, Label};
use gtk::{Button, Box, Orientation, Image, Align, SearchEntry};
use crate::widgets::utils::*;

pub struct SideBar
{
    main_container: Box,
    chats_container: Box,
    search_box: SearchEntry,
    functional_container: Box,
    user_container: Box,
    bottom_container: Box,
    camera_button: Button,
    display_button: Button, // display?
    microphone_button: Button,
    sound_button: Button,
    hung_up_button: Button,
}

impl SideBar
{
    pub fn new() -> Self
    {
        /*Main container: Vertical Box for SideBar*/
        let main_container = Box::new(Orientation::Vertical, 5);
        main_container.set_size_request(200, -1);
        main_container.set_vexpand(true);
        main_container.set_widget_name("side-bar-main-container");
        main_container.set_visible(true);
        
        /*Search box*/
        let search_box=SearchEntry::builder().build();
        search_box.set_widget_name("search-box");
        search_box.set_margin(3);

        /*Subcontainer: Vertical Box for chats*/
        let chats_container=Box::new(Orientation::Vertical, 5);
        chats_container.set_vexpand(true);
        chats_container.set_valign(Align::Start);

        /*Subcontainer: Vertical Box bottom bar*/
        let bottom_container = Box::new(Orientation::Vertical, 10);
        bottom_container.set_valign(Align::End);
        bottom_container.set_widget_name("side-bar-bottom-container");

        /*Subcontainer: Horzontal Box for functional buttons*/
        let functional_container = Box::new(Orientation::Horizontal, 10);
        functional_container.set_valign(Align::End);
        functional_container.set_widget_name("side-bar-functional-container");

        /*Subcontainer: Horzontal Box for info about user*/
        let user_container = Box::new(Orientation::Horizontal, 5);
        user_container.set_vexpand(true);
        user_container.set_valign(Align::End);
        user_container.set_widget_name("side-bar-functional-container");
        
        /*Functional buttons*/
        let camera_button=Button::builder().build();
        camera_button.set_valign(Align::Start);
        let pixbuf = load_image_from_file("icons\\camera_white.png", 30, 30);
        let image = Image::from_pixbuf(Some(&pixbuf));
        camera_button.set_image(Some(&image));
        camera_button.set_always_show_image(true);
        camera_button.set_widget_name("funtional-button");

        let display_button=Button::builder().build();
        display_button.set_valign(Align::Start);
        let pixbuf = load_image_from_file("icons\\display_white.png", 30, 30);
        let image = Image::from_pixbuf(Some(&pixbuf));
        display_button.set_image(Some(&image));
        display_button.set_always_show_image(true);
        display_button.set_widget_name("funtional-button");

        let microphone_button=Button::builder().build();
        microphone_button.set_valign(Align::Start);
        let pixbuf = load_image_from_file("icons\\microphone_white.png", 30, 30);
        let image = Image::from_pixbuf(Some(&pixbuf));
        microphone_button.set_image(Some(&image));
        microphone_button.set_always_show_image(true);
        microphone_button.set_widget_name("funtional-button");

        let sound_button=Button::builder().build();
        sound_button.set_valign(Align::Start);
        let pixbuf = load_image_from_file("icons\\sound_white.png", 30, 30);
        let image = Image::from_pixbuf(Some(&pixbuf));
        sound_button.set_image(Some(&image));
        sound_button.set_always_show_image(true);
        sound_button.set_widget_name("funtional-button");

        let hung_up_button=Button::builder().build();
        hung_up_button.set_valign(Align::Start);
        let pixbuf = load_image_from_file("icons\\hung_up_white.png", 30, 30);
        let image = Image::from_pixbuf(Some(&pixbuf));
        hung_up_button.set_image(Some(&image));
        hung_up_button.set_always_show_image(true);
        hung_up_button.set_widget_name("funtional-button");

        /*Info about user*/
        let user_label = Label::builder().label("Username").build();
        let user_image = Image::from_file("icons\\default_user_white.png");
        user_container.add(&user_image);
        user_container.add(&user_label);
        // user_label.set_widget_name("funtional-button");

        /*Add buttons to subcontainers*/
        functional_container.add(&camera_button);
        functional_container.add(&display_button);
        functional_container.add(&microphone_button);
        functional_container.add(&sound_button);
        functional_container.add(&hung_up_button);
        
        /*Add subcontainers to main container*/
        main_container.add(&search_box);
        main_container.add(&chats_container);
        bottom_container.add(&functional_container);
        bottom_container.add(&user_container);
        main_container.add(&bottom_container);

        let mut side_bar = SideBar
        {
            main_container,
            chats_container,
            search_box,
            functional_container,
            user_container,
            bottom_container,
            camera_button,
            display_button, // display?
            microphone_button,
            sound_button,
            hung_up_button,
        };

        /*Add signals for pressed buttons*/
        side_bar.init_signals();

        side_bar
    }

    fn init_signals(&mut self)
    {
        self.camera_button.connect_clicked(|_| {
            println!("camera_button clicked!");
        });

        self.display_button.connect_clicked(|_| {
            println!("display_button clicked!");
        });

        self.microphone_button.connect_clicked(|_| {
            println!("microphone_button clicked!");
        });

        self.sound_button.connect_clicked(|_| {
            println!("sound_button clicked!");
        });

        self.hung_up_button.connect_clicked(|_| {
            println!("hung_up_button clicked!");
        });
    }

    pub fn widget(&self) -> &gtk::Box
    {
        &self.main_container
    }
}