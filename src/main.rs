use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Задаем время работы программы в секундах
    let duration_seconds = 5;

    // Создаем канал
    let (tx, rx) = mpsc::channel();

    // Создаем поток для отправки данных
    let sender = thread::spawn(move || {
        for i in 0.. {
            tx.send(i).unwrap();
            // Добавим небольшую задержку, чтобы не перегружать канал
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Создаем поток для чтения данных
    let receiver = thread::spawn(move || {
        for received in rx {
            println!("Received: {}", received);
        }
    });

    // Ждем заданное время и затем прерываем потоки
    thread::sleep(Duration::from_secs(duration_seconds));
    println!("Terminating threads...");

    // Ждем завершения потоков
    sender.join().unwrap();
    receiver.join().unwrap();
}