use std::io;
use std::thread;

fn main() {
    println!("Input N:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line"); // считываем N

    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input value not integer"),
    }; // проверяем N на валидность

    let numbers: Vec<i32> = (1..=n).collect(); // Инициализируем массив

    // Получаем количество доступных ядер
    let num_threads = num_cpus::get();
    let chunk_size = (n as usize + num_threads - 1) / num_threads; // размер блока для каждого ядра

    let mut handles = vec![];

    // Разбиваем массив на блоки
    for chunk in numbers.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // Перемещаем данные в поток
        let handle = thread::spawn(move || {
            let results: Vec<i32> = chunk.iter().map(|&num| num * num).collect();
            println!("{:?}", results); // Возврат квадрата
        });

        handles.push(handle);
    }

    // Завершаем все потоки
    for handle in handles {
        handle.join().unwrap();
    }
}
