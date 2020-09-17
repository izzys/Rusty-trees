use ferris_says::say; // from the previous step
use std::io::{stdout,BufWriter};
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    play();
}



fn play() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        
        println!("Guess the number!");

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect(r#"Failed to read line"#);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!(":(") ; continue},
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!");break;},
        }
    }

}
