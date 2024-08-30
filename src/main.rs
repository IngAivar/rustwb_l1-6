use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Задаем время работы программы в секундах
    let duration_seconds = 5;

    // Создаем каналы: один для данных, другой для сигнала о завершении
    let (tx, rx) = mpsc::channel();
    let (tx_stop, rx_stop) = mpsc::channel();

    // Создаем поток для отправки данных
    let sender = thread::spawn(move || {
        for i in 0.. {
            // Проверяем, не пришел ли сигнал о завершении
            if rx_stop.try_recv().is_ok() {
                break;
            }

            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Создаем поток для чтения данных
    let receiver = thread::spawn(move || {
        for received in rx {
            println!("Received: {}", received);
        }
    });

    // Ждем заданное время и затем посылаем сигнал о завершении
    thread::sleep(Duration::from_secs(duration_seconds));
    println!("Terminating threads...");
    tx_stop.send(()).unwrap();

    // Ждем завершения потоков
    sender.join().unwrap();
    receiver.join().unwrap();
}