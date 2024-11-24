use rust_socketio::{Payload, RawClient, ClientBuilder};
use serde_json::json;
use std::io::*;

fn perepishi_vana() {   
    // Callback для обработки сообщений от сервера 
    let callback = |payload: Payload, socket: RawClient| {
        match payload {
            Payload::Text(values) => println!("Received: {:#?}", values),
            Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
            // This payload type is deprecated, use Payload::Text instead
            Payload::String(str) => println!("Received: {}", str),
        }
    };
    // Создание пользователя и подключение к серверу
    let mut socket = ClientBuilder::new("")
        .on("newMessage", callback)
        .on("sendMessage", callback)
        .on("error", |err, _| eprintln!("Client error!: {:#?}", err))
        .connect()
        .expect("connection failed");

    // Считывание информации из консоли 
    let mut input = String::new();  
    let _input = stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");    
    let num_chat = input[0..input.len() - 2].to_string();   
    let mes = json!({"chatId" : num_chat});             
    
    // Отправка сообщения серверу о том чтобы подключиться к чату 
    let res = socket.emit("joinChat", mes);                
    assert!(res.is_ok());  


    loop {           
        // Считывание информации из консоли 
        let mut input = String::new();            
        let _input = stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");      
        input = input[0..input.len() - 2].to_string();   
        let mes = json!({
                        "chatId" : 2,
                        "senderId": 1,
                        "content" : input});

        // Отправка сообщения в чат             
        let res = socket.emit("sendMessage", mes);                
        assert!(res.is_ok());     
    }
}   