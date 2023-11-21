// Пример создания нового потока:
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        // Код, выполняющийся в новом потоке
        println!("Привет из отдельного потока!");
    });
    // Дождаться завершения работы потока
    handle.join().unwrap();
}

// Пример передачи данных между потоками с помощью канала:
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let data = "Привет из отдельного потока!";
        tx.send(data).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("{}", received);

    handle.join().unwrap();
}
// В примере выше мы создали канал с помощью функции mpsc::channel, отправили сообщение из потока отправителя (tx.send(data))
// и получили его в главном потоке (rx.recv()).

// Пример использования мьютекса

use std::sync::Mutex;

fn main() {
    // Создаем мьютекс
    let mutex = Mutex::new(0);

    // Создаем потоки
    let thread1 = std::thread::spawn(|| {
        // Захватываем мьютекс
        let mut data = mutex.lock().unwrap();
        *data += 1;
        println!("Thread 1: {}", *data);
    });

    let thread2 = std::thread::spawn(|| {
        // Захватываем мьютекс
        let mut data = mutex.lock().unwrap();
        *data += 1;
        println!("Thread 2: {}", *data);
    });

    // Ждем завершения потоков
    thread1.join().unwrap();
    thread2.join().unwrap();
}

// В данном примере создается мьютекс mutex, который защищает переменную data.
//Два потока thread1 и thread2 пытаются изменить значение data, но блокируются на момент вызова mutex.lock(), если мьютекс уже захвачен другим потоком.
//Когда мьютекс освобождается от первого потока, то второй поток получает доступ к данным, изменяет их и выводит результат.

// Пример ожидания завершения потока с обработкой ошибок:

use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // Код, выполняющийся в новом потоке
        panic!("Что-то пошло не так!");
    });

    let result = handle.join();
    if let Err(error) = result {
        println!("Произошла ошибка: {:?}", error);
    }
}

// Механизм Result
enum Result<T, E> {
   Ok(T),
   Err(E),
}
// Пример использования
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("path/to/file.txt") {
        Ok(contents) => {
            println!("File contents: {}", contents);
        }
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }
}