 use gtk::prelude::*;
use gtk::{CssProvider, StyleContext};
// use base64::engine::general_purpose::STANDARD;
// use base64::Engine;

// pub fn get_image_from_base64(base64_data: &str) -> gtk::gdk_pixbuf::Pixbuf {
//     // Декодируем данные из Base64
//     let decoded_data = STANDARD.decode(base64_data).unwrap();

//     // Создаем Pixbuf из массива байтов
//     let loader = gtk::gdk_pixbuf::PixbufLoader::new();
//     loader.write(&decoded_data).unwrap();
//     loader.close().unwrap();

//     loader.pixbuf().unwrap()
// }

pub fn load_image_from_file(file_path: &str, width: i32, height: i32) -> gtk::gdk_pixbuf::Pixbuf {
    // Загружаем изображение
    let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_file(file_path).expect("Не удалось загрузить изображение");

    // Масштабируем изображение до нужных размеров
    pixbuf.scale_simple(width, height, gtk::gdk_pixbuf::InterpType::Bilinear).expect("Не удалось изменить размер изображения")
}

pub fn load_css(path: &str) {
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