

const VERSION:u8 = 8;

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
