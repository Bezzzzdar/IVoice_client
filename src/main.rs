use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создаем GTK-приложение
    let app = Application::builder()
        .application_id("com.example.gtk-http")
        .build();

    app.connect_activate(|app| {
        // Создаем главное окно
        let window = ApplicationWindow::builder()
            .application(app)
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

        // Обработчик нажатия кнопки
        button.connect_clicked(|_| {
            // Запускаем асинхронную задачу для отправки HTTP-запроса
            let client = Client::new();
            let payload = json!({
                "username": "testuser",
                "password": "159"
            });

            gtk::glib::MainContext::default().spawn_local(async move {
                match client
                    .post("http://91.122.46.91:3000/auth/login")
                    .json(&payload)
                    .send()
                    .await
                {
                    Ok(response) => {
                        println!("Успех: {}", response.text().await.unwrap_or_default());
                    }
                    Err(err) => {
                        eprintln!("Ошибка: {}", err);
                    }
                }
            });
        });

        // Добавляем кнопку в окно
        window.set_child(Some(&button));
        button.show();
        // Показываем окно
        window.show();
    });

    // Запускаем приложение
    app.run();

    Ok(())
}
