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

    let numbers: Vec<i32> = (1..=n).collect(); // инициализируем массив

    // Получаем количество доступных ядер

    println!("Enter the number of processor cores:");

    let mut num_threads = String::new();

    io::stdin()
        .read_line(&mut num_threads)
        .expect("Failed to read line"); // считываем N

    let num_threads: usize = match num_threads.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input value not integer"),
    };

    let chunk_size = (n as usize + num_threads - 1) / num_threads; // определяем размер блока для каждого ядра

    let mut handles = vec![];

    // Разбиваем массив на блоки
    for chunk in numbers.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // перемещаем данные в поток
        let handle = thread::spawn(move || {
            let results: Vec<i32> = chunk.iter().map(|&num| num * num).collect();
            println!("{:?}", results); // выводим результат вычисления для каждого блока
        });

        handles.push(handle);
    }

    // Завершаем все потоки
    for handle in handles {
        handle.join().unwrap();
    }
}
