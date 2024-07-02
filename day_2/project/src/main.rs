

const VERSION:u8 = 8;
const VERSION2:u8 = 80;
const VERSION3:u8 = 80;
const VERSION4:u8 = 89;
const NEW_VERSION:u8 = 10;

fn main() {
    println!("Tupels :)");

    let emp_info:(&str,u8) = ("Ramish",50);


    println!("{}",emp_info.0);
    println!("{}",emp_info.1);

    // desctructing!
    let (name,age) = emp_info;

    println!("{}",name);
    println!("{}",age);

    println!("{VERSION}");
    print_value();
}

fn print_value(){
    let x = 6;
    println!("{}",x);
    println!("Hi From Function!");

    let some_string = String::from("This is my string as you can see!!");

    println!("{}",some_string);

    // All of the variables be dropped here :)

}

// tupples!!
// tupples are immutable by default
// tupples are fixed size
// tupples can have multiple types
// tupples can have multiple values
// tupples can be destructured
// tupples can be used as function arguments
// tupples can be returned from functions
// tupples can be used in match expressions

// functions!
// ownership!

// create an example of ownership!
fn ownership(){
    let x = 5;
    println!("{}",x);
    // x is dropped here :
}

fn ownership_in_heap(){
    let x = String::from("Hello");
    println!("{}",x);
    // x is dropped here :
}

fn ownership_moved(){
    let x = String::from("Hello");
    let y = x;

    // x cannot me accessed anymore!
    println!("{}",y);
}

fn ownership_borrowed(){
    let x = String::from("Hello");
    let y = &x;

    // x can be accessed here!
    println!("{} {}",x,y);
}


// create an example of functions!

// create an example of tupples!

// create an example of match expressions!

// create an example of destructuring!