// **
// Command Line Calculator or cmdcalc for short
// For those who'd rather not use GUI calculators
//
// @author Austine D. Odhiambo aka Ace Declan
//
// **

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
    let mut result = match args[0].parse::<f64>() {
        Ok(num) => num,
        Err(_) => return None, // Return None if parsing fails
    };
    for arg_str in args {
        match arg_str.parse::<f64>() {
            Ok(arg) => {
                if arg == 0.0 {
                    // Return None if division by zero
                    return None; 
                }

                result /= arg
            },
            Err(_) => return None // Return None if parsing fails
        }
    }

    return Some(result);
}

fn square(args: Vec<String>) -> Option<f64> {
    let mut result: f64 = 1.0;
    for arg_str in args {
        match arg_str.parse::<f64>() {
            Ok(arg) => result = arg * arg,
            Err(_) => return None // Return None if parsing fails
        }
    }

    return Some(result);
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Some(Command::Add{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Error: Subcommand 'add' requires at least one argument.");
                Opt::clap().print_help().expect("Failed to print help");
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match add(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Invalid input provided or division by zero!"),
                }
                //println!("Result = {}", add(args));
            }
        },

        Some(Command::Subtract{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Error: Subcommand 'sub' requires at least one argument.");
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match subtract(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Invalid input provided or division by zero!"),
                }
            }
        },

        Some(Command::Multiply{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Error: Subcommand 'mul' requires at least one argument.");
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match multiply(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Invalid input provided or division by zero!"),
                }
            }
        },

        Some(Command::Divide{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without arguments
                eprintln!("Error: Subcommand 'div' requires at least one argument.");
            }
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match divide(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Invalid input provided or division by zero!"),
                }
            }
        },

        Some(Command::Square{ args }) => {
            if args.is_empty()  {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Error: Subcommand 'sqr' requires at least one argument.");
            }

            else if args.len() > 1 {
                // Error handling in case user enters subcommand without arguments
                eprintln!("Error: Subcommand 'sqr' requires a maximum of one argument.");
            }   
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match square(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Invalid input provided or division by zero!"),
                }
            }
        },

        None => {
            // Handle the case when no subcommand is specified
            // Display the help message
            Opt::clap().print_help().expect("Failed to print help");
        }
    }
}