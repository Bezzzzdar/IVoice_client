use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};


fn perepishi_vanja() -> Result<(), Box<dyn std::error::Error>> {

    let host = cpal::default_host();

    // Устройства
    let input_device  = host.default_input_device().expect("No input device found");
    let output_device = host.default_output_device().expect("No output device found");

    // Конфигурация
    let input_config = input_device.default_input_config()?.config();
    println!("Input config: {:?}", input_config);
    let output_config = output_device.default_output_config()?.config();
    println!("Output config: {:?}", output_config);

    for config in input_device.default_input_config() {
        println!("{:?}", config);
    }
    for config in output_device.default_output_config() {
        println!("{:?}", config);
    }

    // Параметры фильтра
    let sample_rate = input_config.sample_rate.0 as f32;
    let cutoff_frequency = 500.0; // Частота среза фильтра (например, 500 Гц)
    let alpha = 1.0 / (1.0 + (sample_rate / (2.0 * std::f32::consts::PI * cutoff_frequency)).tan());

    let mut prev_output = 0.0;


    if input_config.sample_rate != output_config.sample_rate {
        println!("Rate micro {:?}", input_config.sample_rate);
        println!("Rate Dinamic {:?}", output_config.sample_rate );
        return Err("Input and output sample rates do not match".into());
    }

    // Общий буфер
    let audio_buffer = Arc::new(Mutex::new(Vec::new()));

    // Поток ввода
    let input_buffer = Arc::clone(&audio_buffer);
    let input_stream = input_device.build_input_stream(
        &input_config,
        move |data: &[f32], _| {
            let mut buffer = input_buffer.lock().unwrap();
            if buffer.len() > 48000 * 2 {
                buffer.drain(0..data.len());
            }
            buffer.extend_from_slice(data);
        },
        move |err| eprintln!("Input stream error: {}", err),
        None
    )?;

    // Поток вывода
    let output_buffer = Arc::clone(&audio_buffer);
    let output_stream = output_device.build_output_stream(
        &output_config,
        move |output: &mut [f32], _| {
            let mut buffer = output_buffer.lock().unwrap();
            for frame in output.chunks_mut(8) {
                if let Some(&sample) = buffer.get(0) {
                    // Применяем низкочастотный фильтр
                    let filtered_sample = alpha * sample + (1.0 - alpha) * prev_output;
                    prev_output = filtered_sample;

                    // Дублируем фильтрованный сигнал в 8 каналов
                    for channel in frame.iter_mut() {
                        *channel = filtered_sample;
                    }

                    buffer.remove(0); // Удаляем использованный образец
                } else {
                    // Если данных нет, выводим тишину
                    for channel in frame.iter_mut() {
                        *channel = 0.0;
                    }
                }
            }
        },
        move |err| eprintln!("Output stream error: {}", err),
        None
    )?;

    // Запускаем потоки
    input_stream.play()?;
    output_stream.play()?;
    println!("Streaming audio...");

    // Удерживаем приложение
    std::thread::park();

    Ok(())

}