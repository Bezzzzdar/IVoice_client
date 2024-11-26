use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation};
use crate::widgets::server_bar::ServerBar;
use crate::widgets::side_bar::SideBar;
use crate::widgets::utils::*;


pub struct MainWindow {
    window: ApplicationWindow,
    server_bar: ServerBar,
    side_bar: SideBar,
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
        window.set_size_request(1000, 700);

        let global_frame = Box::new(Orientation::Horizontal, 0);

        let server_bar = ServerBar::new();
        let side_bar = SideBar::new();
        
        global_frame.add(server_bar.widget());
        global_frame.add(side_bar.widget());

        /*Load css for all widgets*/
        load_css("src\\styles\\styles.css");
        window.set_child(Some(&global_frame));
        window.show_all();

        MainWindow {
            window,
            server_bar,
            side_bar,
        }
    }

    pub fn show(&self) {
        self.window.show();
    }
}
