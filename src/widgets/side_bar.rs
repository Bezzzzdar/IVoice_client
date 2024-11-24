use gtk::prelude::*;
use gtk::{Revealer, Button, Box, Orientation, Label};

#[derive(Clone)]
pub struct SideBar {
    revealer: Revealer,
    close_button: Button,
}

impl SideBar {
    pub fn new() -> Self {
        let revealer = Revealer::builder()
            .transition_type(gtk::RevealerTransitionType::SlideRight)
            .reveal_child(false)
            .build();

        let close_button = Button::builder().label("Закрыть").build();

        SideBar {
            revealer,
            close_button,
        }
    }

    pub fn init_ui(&self) {
        let sidebar = Box::new(Orientation::Vertical, 5);
        let label = Label::new(Some("Это выезжающее меню"));

        sidebar.add(&label);
        sidebar.add(&self.close_button);

        self.revealer.set_child(Some(&sidebar));
    }

    pub fn set_visible(&self, visible: bool) {
        self.revealer.set_reveal_child(visible);
    }

    pub fn is_visible(&self) -> bool {
        self.revealer.reveals_child()
    }

    pub fn widget(&self) -> &impl gtk::prelude::IsA<gtk::Widget> {
        &self.revealer
    }

    pub fn connect_close_signal<F: Fn() + 'static>(&self, callback: F) {
        self.close_button.connect_clicked(move |_| {
            callback();
        });
    }
}
