use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
mod utils;

struct MyApp {
    app: gtk::Application,
}

impl MyApp {
    /// Создание нового приложения
    fn new(app_id: &str) -> Self {
        let app = Application::builder()
            .application_id(app_id)
            .build();
        Self { app }
    }

    /// Настройка интерфейса
    fn init_ui(&self) {
        // Создаем главное окно
        let window = ApplicationWindow::builder()
            .application(&self.app)
            .title("HTTP Request Example")
            .default_width(400)
            .default_height(200)
            .build();

        // Создаем кнопку
        let button = Button::builder()
            .label("Отправить запрос")
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .build();

        // Подключаем обработчик нажатия
        self.connect_button_clicked(&button);

        // Добавляем кнопку в окно
        window.set_child(Some(&button));
        button.show();

        // Показываем окно
        window.show();
    }

    /// Обработчик нажатия кнопки
    fn connect_button_clicked(&self, button: &Button) {
        button.connect_clicked(|_| { utils::auth::auth_reqwest("testuser", "159");});
    }

    /// Запуск приложения
    fn run(&self) {
        let app = self.app.clone();
        app.connect_activate(move |app| {
            let my_app = MyApp { app: app.clone() };
            my_app.init_ui();
        });

        self.app.run();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создаем и запускаем приложение
    let app = MyApp::new("com.example.gtk-http");
    app.run();

    Ok(())
}
