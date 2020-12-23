use std::io;

fn main() {
    println!("hello world");
    println!("guess the number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("line read failed");

    println!("you guessed {}", guess);
}
