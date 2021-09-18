use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("❓❓❓❓❓❓❓❓❓");
    println!("❓ Угадай число ❓");
    println!("❓❓❓❓❓❓❓❓❓");
    println!("\nНужно ввести число от 1 до 100");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut attempts = 0;

    loop {
        attempts += 1;

        println!("\nВведи число (попытка {}):", attempts);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!"),
            Ordering::Greater => println!("Слишком большое!"),
            Ordering::Equal => {
                println!("\n✨✨✨✨✨✨✨");
                println!("✨ Победа! ✨");
                println!("✨✨✨✨✨✨✨");
                break;
            },
        }
    }
}
