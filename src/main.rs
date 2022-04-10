use std::io::*;


fn main() {
    greet();
    let x = category_taker();
}

fn greet() {
    println!("Welcome to the unit conversion program created by LordOfWizard");
    println!("follow on github.com ");
    println!("https://github.com/lordofwizard");
    println!("for more information about this project ⬇");
    println!("https://github.com/lordofwizard/unit.git");
}

fn category_taker() -> category{
    println!("Please input a category you want to do conversions on ");
    println!("1. Temprature\n2. Mass");


    // Taking u8 input
    
    let mut msg : String = String::new();
    std::io::stdin().read_line(&mut msg).expect("Given input is not actually taken.. Some error occured");
    let input : u8 = match msg.trim().parse(){
        Ok(u) => u,
        Err(x) => 0
    };
    if input == 1 {
        category::Temprature
    }
    else if input == 2 {
        category::Mass
    }
    else {
        category::No
    }

}
enum category{
    Temprature,
    Mass,
    No
}