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
// ** Errors resulting from the program panicing should be marked as application errors
//
// ** **

use std::option::Option;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cmdcalc", about = "Command line calculator")]
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

    #[structopt(name = "sqr", about = "Squares the provided number")]
    Square {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "pow", about = "Takes the first argument and raises it to the power of the second")]
    Power {
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
    let arg_1: f64 = match args[0].parse::<f64>() {
        Ok(arg_1) => arg_1,
        Err(_) => return None
    };
    let arg_2: f64 = match args[1].parse::<f64>() {
        Ok(arg_2) => arg_2,
        Err(_) => return None
    };

    if arg_1.is_nan() || arg_2.is_nan() {
        return None;
    }

    if arg_1 == 0.0 || arg_2 == 0.0 {
        return None;
    }

    else { 
        let result: f64 = arg_1 / arg_2;
        return Some(result);
    }
}

// Square function
fn square(args: Vec<String>) -> Option<f64> {
    let arg: f64 = match args[0].parse::<f64>(){
        Ok(arg) => arg,
        Err(_) => return None
    };

    if arg.is_nan(){
        return None;
    }

    else {
        let result = arg * arg;
        return Some(result);
    }
    
}

// Power function
fn power(args: Vec<String>) -> Option<f64> {
    let base: f64 = match args[0].parse::<f64>() {
        Ok(base) => base,
        Err(_) => return None
    };

    let exp: f64 = match args[1].parse::<f64>() {
        Ok(base) => base,
        Err(_) => return None
    };

    if base.is_nan() || exp.is_nan() {
        return None;
    }

    else {
        let result: f64 = base.powf(exp);
        return Some(result);
    }
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        // Add subcommand
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


        // Subtract subcommand
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


        // Multiply subcommand
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


        // Divide subcommand
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
                    None => println!("Usage Error: An invalid input value (eg a non-numerical value) was provided OR you attempted to divide by zero!"),
                }
            }
        },


        // Square subcommand
        Some(Command::Square{ args }) => {

            if args.is_empty()  {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Usage Error: Subcommand 'sqr' requires at least one argument.");
            }

            else if args.len() > 1 {
                // Error handling in case user enters subcommand without arguments
                eprintln!("Usage Error: Subcommand 'sqr' requires a maximum of one argument.");
            }
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match square(args){
                    Some(result) => println!("Result: {}", result),
                    None => println!("Usage Error: An invalid input value was provided"),
                }
            }
        },
        

        // Power subcommand
        Some(Command::Power{ args }) => {
            if args.is_empty()  {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!("Usage Error: Subcommand 'pow' requires at least one argument.");
            }

            else if args.len() < 2 {
                // Error handling in case user enters subcommand without arguments
                eprintln!("Usage Error: Subcommand 'pow' requires a minimum of two arguments.");
            }

            else if args.len() > 2 {
                // Error handling in case user enters subcommand without arguments
                eprintln!("Usage Error: Subcommand 'pow' requires a maximum of two arguments.");
            }
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match power(args) {
                    Some(result) => println!("Result: {}", result),
                    None => println!("Usage Error: An invalid input value was provided"),
                }
            }
        },


        None => {
            // Handle the case when no subcommand is specified
            // Display the help message
            Opt::clap().print_help().expect("Application Error: Failed to print help");
        }
    }
}