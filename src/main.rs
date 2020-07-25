use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Lets play number guess game");
    let randno = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("give no");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input);
    
        let no : u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Invalid input, please try again"); continue; }
        }; 

        match no.cmp(&randno) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => { println!("on target"); break; }
        }
    }
}
