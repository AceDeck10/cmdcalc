// **
//
// Command Line Calculator or cmdcalc for short
// For those who'd rather not use GUI calculators
//
// @author Austine D. Odhiambo aka Ace Declan
//
// ** Errors: The program genarates two types of errors
// **** Usage Errors or Application Errors
// **** Errors resulting from wrong usage by the user should be marked as usage errors
// **** Errors resulting from the program panicing should be marked as application errors
//
// ** **

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cmdcalc", about = "Command line calculator")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add", about = "Add: Performs addition on the provided numbers")]
    Add {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "sub", about = "Subtract: Performs subtruction on the provided numbers IN THE ORDER PROVIDED")]
    Subtract {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "mul", about = "Multiply: Performs multiplication on the provided numbers")]
    Multiply {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "div", about = "Divide: Performs division on the provided numbers IN THE ORDER PROVIDED")]
    Divide {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "sqr", about = "Square: Squares the provided number")]
    Square {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "sqrt", about = "Square root: Squares the provided number")]
    SquareRoot {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "pow", about = "Power: Takes the first argument and raises it to the power of the second")]
    Power {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },

    #[structopt(name = "add", about = "Mean: Performs addition on the provided numbers the divides the result by the number of rlements")]
    Mean {
        #[structopt(name = "args", min_values = 1)]
        args: Vec<String>,
    },
}

// Add the provided parameters
fn add(args: &Vec<String>) -> Option<f64> {
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
fn subtract(args: &Vec<String>) -> Option<f64> {
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
fn multiply(args: &Vec<String>) -> Option<f64>{
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
fn divide(args: &Vec<String>) -> Option<f64> {
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
fn square(args: &Vec<String>) -> Option<f64> {
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

// Square root function
fn square_root(args: &Vec<String>) -> Option<f64> {
    let arg: f64 = match args[0].parse::<f64>(){
        Ok(arg) => arg,
        Err(_) => return None
    };

    if arg.is_nan(){
        return None;
    }

    else {
        let result = arg.sqrt();
        return Some(result);
    }
} 

// Power function
fn power(args: &Vec<String>) -> Option<f64> {
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

// Mean function
fn mean(args: &Vec<String>) -> Option<f64> {
    let mut result: f64 = 0.0;
    for arg_str in args {
        match arg_str.parse::<f64>() {
            Ok(arg) => (result += arg) / args.len(),
            Err(_) => return None // Return None if parsing fails
        };
    }

    return Some(result);
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        // Add subcommand
        Some(Command::Add{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!(" Function: Add \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'add' requires at least one argument.", args);
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match add(&args) {
                    Some(result) => println!(" Function: Add \n Input(s): {:?} \n Result: {}", args, result),
                    None => println!(" Function: Add \n Input(s): {:?}  \n Result: Usage Error: An invalid input was provided.", args),
                }
            }
        },


        // Subtract subcommand
        Some(Command::Subtract{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!(" Function: Subtract \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'sub' requires at least one argument.", args);
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match subtract(&args) {
                    Some(result) => println!(" Function: Subtract \n Input(s): {:?} \n Result: {}", args, result),
                    None => println!(" Function: Subtract \n Input(s): {:?}  \n Result: Usage Error: An invalid input was provided", args),
                }
            }
        },


        // Multiply subcommand
        Some(Command::Multiply{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!(" Function: Multiply \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'mul' requires at least one argument.", args);
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match multiply(&args) {
                    Some(result) => println!(" Function: Multiply \n Input(s): {:?} \n Result: {}", args, result),
                    None => println!(" Function: Multiply \n Input(s): {:?}  \n Result: Usage Error: An invalid input was provided", args),
                }
            }
        },


        // Divide subcommand
        Some(Command::Divide{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without arguments
                eprintln!(" Function: Divide \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'div' requires at least two arguments.", args);
            }

            else if args.len() < 2 {
                // Error handling in case user enters subcommand with one argument
                eprintln!(" Function: Divide \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'div' requires at least two arguments.", args);
            }

            else if args.len() > 2 {
                // Error handling in case user enters subcommand with more than two arguments
                eprintln!(" Function: Divide \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'div' requires a maximum of two arguments. Can only divide two numbers at time", args);
            }
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match divide(&args) {
                    Some(result) => println!(" Function: Divide \n Input(s): {:?} \n Result: {}", args, result),
                    None => println!(" Function: Divide \n Input(s): {:?}  \n Result: Usage Error: An invalid input value (eg a non-numerical value) was provided OR you attempted to divide by zero!", args),
                }
            }
        },


        // Square subcommand
        Some(Command::Square{ args }) => {

            if args.is_empty()  {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!(" Function: Square \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'sqr' requires at least one argument.", args);
            }

            else if args.len() > 1 {
                // Error handling in case user enters subcommand without arguments
                eprintln!(" Function: Square \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'sqr' requires a maximum of one argument.", args);
            }
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match square(&args){
                    Some(result) => println!(" Function: Square \n Input(s): {:?} \n Result: {}", args, result),
                    None => println!(" Function: Square \n Input(s): {:?}  \n Result: Usage Error: An invalid input value was provided", args),
                }
            }
        },


        // Square root subcommand
        Some(Command::SquareRoot{ args }) => {

            if args.is_empty()  {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!(" Function: Square root \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'sqrt' requires at least one argument.", args);
            }

            else if args.len() > 1 {
                // Error handling in case user enters subcommand without arguments
                eprintln!(" Function: Square root \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'sqrt' requires a maximum of one argument.", args);
            }
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match square_root(&args){
                    Some(result) => println!(" Function: Square root \n Input(s): {:?} \n Result: {}", args, result),
                    None => println!(" Function: Square root \n Input(s): {:?}  \n Result: Usage Error: An invalid input value was provided", args),
                }
            }
        },
        

        // Power subcommand
        Some(Command::Power{ args }) => {
            if args.is_empty()  {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!(" Function: power \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'pow' requires at least one argument.", args);
            }

            else if args.len() < 2 {
                // Error handling in case user enters subcommand without arguments
                eprintln!(" Function: Power \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'pow' requires a minimum of two arguments.", args);
            }

            else if args.len() > 2 {
                // Error handling in case user enters subcommand without arguments
                eprintln!(" Function: Power \n Input(s): {:?}  \n Result:Usage Error: Subcommand 'pow' requires a maximum of two arguments.", args);
            }
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match power(&args) {
                    Some(result) => println!(" Function: Power \n Input(s): {:?} \n Result: {}", args, result),
                    None => println!(" Function: Power \n Input(s): {:?}  \n Result:Usage Error: An invalid input value was provided", args),
                }
            }
        },

        // Mean subcommand
        Some(Command::Mean{ args }) => {
            if args.is_empty() {
                // Error handling in case user enters subcommand without providing arguments
                eprintln!(" Function: Add \n Input(s): {:?}  \n Result: Usage Error: Subcommand 'add' requires at least one argument.", args);
            } 
            
            else {
                // Parse arguments to appropriate function for evaluation
                // and print out result
                match mean(&args) {
                    Some(result) => println!(" Function: Add \n Input(s): {:?} \n Result: {}", args, result),
                    None => println!(" Function: Add \n Input(s): {:?}  \n Result: Usage Error: An invalid input was provided.", args),
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