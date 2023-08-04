//use std::io::{stdin};

use structopt::StructOpt;

/// Command line calculator
#[derive(Debug, StructOpt)]
#[structopt(name = "", about = "A command line calculator")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add", about = "Performs addition on the provided numbers")]
    Add {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<i32>,
    },

    #[structopt(name = "sub", about = "Performs subtruction on the provided numbers")]
    Subtract {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<i32>,
    },

    #[structopt(name = "mul", about = "Performs mutltiplication on the provided numbers")]
    Multiply {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<i32>,
    },

    #[structopt(name = "div", about = "Performs division on the provided numbers")]
    Divide {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<i32>,
    },
}

// Add the provided parameters
fn add(args: Vec<i32>) {
    let mut result: i32 = 0;
    for arg in args {
        result += arg;
    }

    println!("Result = {}", result);
}

// Subtruct the provided parameters
fn subtract(args: Vec<i32>) {
    let mut result: i32 = 0;
    for arg in args {
        result -= arg;
    }

    println!("Result = {}", result);
}

// Multiply the provided parameters
fn multiply(args: Vec<i32>) {
    let mut result: i32 = 0;
    for arg in args {
        result *= arg;
    }

    println!("Result = {}", result);
}

// Divide the provided parameters
fn divide(args: Vec<i32>) {
    let mut result: i32 = 0;
    for arg in args {
        result /= arg;
    }

    println!("Result = {}", result);
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Add{ args } => {
            add(args);
        }

        Command::Subtract{ args } => {
            subtract(args);
        }

        Command::Multiply{ args } => {
            multiply(args);
        }

        Command::Divide{ args } => {
            divide(args);
        }
    }

    /*let mut str_num_1 = String::new();
    let mut str_num_2 = String::new();
    let mut str_operation = String::new();
    let result;

    println!("Enter first number (Larger number first for subtraction and division): ");
    stdin().read_line(&mut str_num_1).expect("Invalid value entered");

    println!("Enter second number: ");
    stdin().read_line(&mut str_num_2).expect("Invalid value entered");

    println!("Enter arithmetic operator: ");
    stdin().read_line(&mut str_operation).expect("Invalid operator entered");

    let num_1: i32 = str_num_1.trim().parse().expect("Value entered is not a number");
    let num_2: i32 = str_num_2.trim().parse().expect("Value entered is not a number");
    let operation = str_operation.trim();

    match operation{
        "add"=>{
            result = num_1 + num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "plus"=>{ 
            result = num_1 + num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "+" =>{ 
            result = num_1 + num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "sub"=>{ 
            result = num_1 - num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "subtract"=>{ 
            result = num_1 - num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "minus"=>{ 
            result = num_1 - num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "-"=>{ 
            result = num_1 - num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "mult"=>{ 
            result = num_1 * num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "multiply"=>{ 
            result = num_1 * num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "times"=>{ 
            result = num_1 * num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);           
        },
        "*"=>{ 
            result = num_1 * num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "div"=>{ 
            result = num_1 / num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "divide"=>{ 
            result = num_1 / num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        "/"=>{ 
            result = num_1 / num_2;
            println!("{} {} {} is {}", num_1, operation, num_2, result);
        },
        _ => println!("Invalid operator")
    }*/ 
}
