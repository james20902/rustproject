use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Lets play number guess game");
    let randno = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("give no");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {println!("{}", input);},
            Err(_) => { println!("readline failed!"); break;}
        };
    
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
    println!("{ }", square(2));
    exampleLogic(1);
}

//functions
//this is a function with a parameter and a return value that only contains an expression
fn square(x: i32) -> i32{
    x * x
}

//functions are made up of statements and will typically end with an expression
//statements perform an action, and expressions will return a value
//ex: let mut y = getexpression() + 5 is a statement, it doesn't return a final value
//ex: getexpression() + 5 in itself is an expression, this DOES return a final value
//expressions do not end with semicolons, it will turn into a statement otherwise, it must be
//treated as a literal value

//logic flow
//pretty basic
fn exampleLogic(x: i32){
    if x == 1 {
        println!("one");
    } else if x == 2 {
        println!("two");
    } else if x == 3 {
        println!("three");
    } else {
        //ternary operator equivalent
        //this works because the whole if else block is an expression which the statement accepts
        //let val = if true { 5 } else { 6 };
        println!("no more in dictionary");
    }

    //loops can be expressions
    let mut counter = 0;
    let val = loop {
        counter += 1;
        if counter >= 10{
            break counter;
        }
    };
    println!("{ }", val);

    //conditional while is the same
    while counter < 15{
        counter += 1;
    }
    println!("{ }", counter);

    let arr : [u32; 5] = [1, 2, 3, 4, 5];
    //for loop works like python
    //for <value> in <set>
    for x in arr.iter(){
        println!("{ }", x);
    }
}
