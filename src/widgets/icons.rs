// use gtk::prelude::*;
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
