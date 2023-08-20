// **
//
// Command Line Calculator or cmdcalc for short
// For those who'd rather not use GUI calculators
//
// @author Austine D. Odhiambo aka Ace Declan
//
// Errors: The program genarates two types of errors
// ** Usage Errors or Application Errors
// ** Errors resulting from wrong usage by the user should be marked as usage errors
// ** Errors resulting from the program panicing should ge marked as application errors
//
// ** **

use std::option::Option;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cmdcalc", about = "A command line calculator")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add", about = "Performs addition on the provided numbers")]
    Add {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "sub", about = "Performs subtruction on the provided numbers IN THE ORDER PROVIDED")]
    Subtract {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "mul", about = "Performs mutltiplication on the provided numbers")]
    Multiply {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "div", about = "Performs division on the provided numbers IN THE ORDER PROVIDED")]
    Divide {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "sqr", about = "Squares on the provided number")]
    Square {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },
}

// Add the provided parameters
fn add(args: Vec<String>) -> Option<f64> {
    let mut result: f64 = 0.0;
    for arg_str in args {
        match arg_str.parse::<f64>() {
            Ok(arg) => result += arg,
            Err(_) => return None // Return None if parsing fails
        };
    }

    return Some(result);
}

// Subtruct the provided parameters
fn subtract(args: Vec<String>) -> Option<f64> {
    let mut result: f64 = match args[0].parse::<f64>() {
        Ok(num) => num,
        Err(_) => return None // Return None if parsing fails
    };

    for arg_str in &args[1..] {
        match arg_str.parse::<f64>() {
            Ok(arg) => result -= arg.abs(),
            Err(_) => return None // Return None if parsing fails
        }
        
    }

    return Some(result);
}

// Multiply the provided parameters
fn multiply(args: Vec<String>) -> Option<f64>{
    let mut result: f64 = 1.0;
    for arg_str in args {
        match arg_str.parse::<f64>() {
            Ok(arg) => result *= arg,
            Err(_) => return None // Return None if parsing fails
        }
    }

    return Some(result);
}

// Divide the provided parameters
fn divide(args: Vec<String>) -> Option<f64> {
    let arg_1 = args[0].parse::<f64>().unwrap();
    let arg_2 = args[1].parse::<f64>().unwrap();

    if arg_1 == 0.0 || arg_2 == 0.0 {
        return None;
    }

    else { 
        let result: f64 = arg_1 / arg_2;
        return Some(result);
    }
}

// Square function
fn square(args: Vec<String>) -> f64 {
    let arg = args[0].parse::<f64>().unwrap();
    let result = arg * arg;
    return result;
}

// power function
/*fn power(args: Vec<String>) -> f64 {
    let base = args[0].parse::<f64>().unwrap();
    let num = args[1].parse::<f64>().unwrap();
    let exp = args[2].parse::<f64>().unwrap();  
}*/

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Some(Command::Add{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Usage Error: Subcommand 'add' requires at least one argument.");
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match add(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Usage Error: An invalid input was provided"),
                }
            }
        },

        Some(Command::Subtract{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Usagr Error: Subcommand 'sub' requires at least one argument.");
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match subtract(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Usage Error: An invalid input was provided"),
                }
            }
        },

        Some(Command::Multiply{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Usage Error: Subcommand 'mul' requires at least one argument.");
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match multiply(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Usage Error: An invalid input was provided"),
                }
            }
        },

        Some(Command::Divide{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without arguments
                eprintln!("Usage Error: Subcommand 'div' requires at least two arguments.");
            }

            else if args.len() < 2 {
                // Error handling in case user enters subcommand with one argument
                eprintln!("Usage Error: Subcommand 'div' requires at least two arguments.");
            }

            else if args.len() > 2 {
                // Error handling in case user enters subcommand with more than two arguments
                eprintln!("Usage Error: Subcommand 'div' requires a maximum of two arguments. Can only divide two numbers at time");
            }
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match divide(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Usage Error: An invalid input value was provided or you attempted to divide by zero!"),
                }
            }
        },

        Some(Command::Square{ args }) => {
            let sqr_arg  = args[0].parse::<f64>().expect("Error: Invalid argument");

            if args.is_empty()  {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Usage Error: Subcommand 'sqr' requires at least one argument.");
            }

            else if args.len() > 1 {
                // Error handling in case user enters subcommand without arguments
                eprintln!("Usage Error: Subcommand 'sqr' requires a maximum of one argument.");
            }

            else if sqr_arg.is_nan() {
                // Program panics before getting to this stage. Will fix later
                // Error handling in case user enters string that cannot be converted to f64
                eprintln!("Usage Error: Subcommand 'sqr' requires a maximum of one argument.");
            }   
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                println!("Result: {}", square(args));
            }
        },

        None => {
            // Handle the case when no subcommand is specified
            // Display the help message
            Opt::clap().print_help().expect("Application Error: Failed to print help");
        }
    }
}