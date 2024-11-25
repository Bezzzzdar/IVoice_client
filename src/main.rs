mod widgets;

use gtk::prelude::*;
use gtk::Application;
use widgets::main_window::MainWindow;
// mod utils;
// mod widgets
// {
//     pub mod main_window;
// }

// struct App 
// {
//     app: gtk::Application,
// }

// impl App 
// {
//     /// Создание нового приложения
//     fn new(app_id: &str) -> Self {
//         let app = Application::builder()
//             .application_id(app_id)
//             .build();
//         Self { app }
//     }

//     /// Настройка интерфейса
//     fn init_ui(&self) {
//         // Создаем главное окно
//         let window = ApplicationWindow::builder()
//             .application(&self.app)
//             .title("HTTP Request Example")
//             .default_width(400)
//             .default_height(200)
//             .build();

//         // Создаем кнопку
//         let button = Button::builder()
//             .label("Отправить запрос")
//             .margin_top(20)
//             .margin_bottom(20)
//             .margin_start(20)
//             .margin_end(20)
//             .build();

//         // Подключаем обработчик нажатия
//         self.connect_button_clicked(&button);

//         // Добавляем кнопку в окно
//         window.set_child(Some(&button));
//         button.show();

//         // Показываем окно
//         window.show();
//     }

//     /// Обработчик нажатия кнопки
//     fn connect_button_clicked(&self, button: &Button) {
//         button.connect_clicked(|_| { utils::auth::auth_reqwest("testuser", "159");});
//     }

//     /// Запуск приложения
//     fn run(&self) {
//         let app = self.app.clone();
//         app.connect_activate(move |app| {
//             let my_app = App { app: app.clone() };
//             my_app.init_ui();
//         });

//         self.app.run();
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Создаем и запускаем приложение
//     let app = App::new("com.example.gtk-http");
//     app.run();

//     Ok(())
// }


fn main()
{
    let app = Application::builder()
        .application_id("com.example.gtk-multiple-classes")
        .build();

    app.connect_activate(|app|{
        let main_window = MainWindow::new(app);
        main_window.show();
    });

    app.run();
}