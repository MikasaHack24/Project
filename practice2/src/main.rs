use std::io;

fn main() {
    loop{
    println!("Choose an operation");
    println!("1. Basic Calculator");
    println!("2. Basic CGPA calculator");
    println!("3. Exit");

    let mut choice = String::new();
    io::stdin().read_line(& mut choice).expect("Failed to read line");
    let choice :i32 = choice.trim().parse().expect("Invalid operation");

    match choice {
        1 => bascic_calculator(),
        //2 => cgpa_calculator(),
        3 => break,
        _ => println!("invalid choice"),

        
    }
    
}
}

fn bascic_calculator(){
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice:i32 = choice.trim().parse().expect("invalid choice");

    println!("Enter first number");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to readline");
    let num1 :f64 = num1.trim().parse().expect("invalid number");

    println!("Enter operator");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let operator : char = operator.trim().parse().expect("invalid operator");

    println!("Enter second number");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2:f64 = num2.trim().parse().expect("Failed to readline");

    let sum: f64 = num1 + num2;
    let subtract_num: f64 = num1 - num2;
    let mult_num :f64 = num1 * num2;
    let div_num : f64 = num1 / num2;
    if num1 == 0.0{
        println!("Error by zero division");
    }


    match choice {
        1 => println!("{}", sum),
        2 => println!("{}", subtract_num),
        3 => println!("{}", mult_num),
        4 => println!("{}", div_num),

        _  =>println!("invalid choice"),
        
    }
}

