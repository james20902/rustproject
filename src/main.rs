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

    //mutability and shadowing!
    //constant
    const no1 : u8 = 32;
    //immutable variable
    let no2 = "something";
    //mutable variable
    let mut no3 = "eee err";

    //first can never be modified and needs explicit type at compile time, second can be "shadowed" as such:
    let no2 = no2.trim();
    //shadowed meaning it references and reassigns using itself
    //last can be changed as usual
    
    //data types!
    //two different types, scalar types and compound types
    
    //scalar
    //integer from 8-128bits or arch which defaults to OS
    let scal1 : u32 = 2736198;
    //floating point which defaults to f64, can use f32
    let scal2 : f64 = 3.1415926535897932382;
    //boolean 1 bit
    let scal3 : bool = false;
    //let scal3 = false;
    //char 8 bit
    let scal4 : char = 'E';

    //compound
    //tuples like python
    let comp1 : (u32, f64, bool) = (12345, 19.21, true);
    //access like array with index
    println!("tuple integer: { }", comp1.0);

    //arrays like everything else
    let comp2 : [u32; 3] = [1, 2, 3];
    println!("array integer: { }", comp2[2]);

}
