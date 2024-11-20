use reqwest::Client;
use gtk::prelude::*;
use serde_json::json;

pub fn auth_reqwest(username: &str, password: &str)
{
    let client = reqwest::Client::new();
    let reqwest_body = json!({
        "username": format!("{}", username),
        "password": format!("{}", password)
    });

    gtk::glib::MainContext::default().spawn_local(async move {
        match client
            .post("http://91.122.46.91:3000/auth/login")
            .json(&reqwest_body)
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

}
