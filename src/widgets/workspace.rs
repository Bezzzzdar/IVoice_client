use std::default;

use gtk::{prelude::*, Label};
use gtk::{Button, Box, Orientation, Image, Align};
use crate::widgets::utils::*;

pub struct WorkSpace
{
    main_container: Box,
    default_label: Label,
}

impl WorkSpace {
    pub fn new() -> Self
    {
        let main_container = Box::new(Orientation::Vertical, 5);
        main_container.set_vexpand(true);
        main_container.set_hexpand(true);
        main_container.set_widget_name("workspace-main-container");
        main_container.set_visible(true);
        main_container.set_valign(Align::Center);

        let default_label = Label::builder().label("Select chat or server to start messaging").build();
        main_container.add(&default_label);

        let mut workspace = WorkSpace 
        { 
            main_container,
            default_label,
        };

        workspace
    }

    // fn init_signals(&mut self)
    // {

    // }

    pub fn widget(&self) -> &gtk::Box
    {
        &self.main_container
    }
}