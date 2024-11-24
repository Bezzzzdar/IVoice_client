use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Orientation};
use crate::widgets::side_bar::SideBar;

pub struct MainWindow {
    window: ApplicationWindow,
    toggle_button: Button,
    side_bar: SideBar,
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Выезжающее окно слева")
            .default_width(600)
            .default_height(400)
            .build();

        let toggle_button = Button::builder().label("Меню").build();
        let side_bar = SideBar::new();
        
        MainWindow {
            window,
            toggle_button,
            side_bar,
        }
    }

    pub fn init_ui(&self) {
        self.side_bar.init_ui();

        let container = Box::new(Orientation::Horizontal, 0);
        container.add(self.side_bar.widget());
        container.add(&self.toggle_button);

        self.window.set_child(Some(&container));
        self.window.show_all();
    }

    pub fn setup_signals(&self) {
        let side_bar = self.side_bar.clone();
        self.toggle_button.connect_clicked(move |_| {
            let current_state = side_bar.is_visible();
            side_bar.set_visible(!current_state);
        });

        let side_bar = self.side_bar.clone();
        self.side_bar.connect_close_signal(move || {
            side_bar.set_visible(false);
        });
    }

    pub fn show(&self) {
        self.window.show();
    }
}
