// *
// Command Line Calculator or cmdcalc for short
// For those who'd rather not use GUI calculators
//
// @author Austine D. Odhiambo aka Ace Declan
// *

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cmdcalc", about = "A command line calculator")]
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
fn add(args: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for arg in args {
        result += arg;
    }

    return result;
}

// Subtruct the provided parameters
fn subtract(args: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for arg in args {
        result -= arg;
    }

    return result;
}

// Multiply the provided parameters
fn multiply(args: Vec<i32>) -> i32{
    let mut result: i32 = 0;
    for arg in args {
        result *= arg;
    }

    return result;
}

// Divide the provided parameters
fn divide(args: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for arg in args {
        result /= arg;
    }

    return result;
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Add{ args } => {
            println!("Result = {}", add(args));
        }

        Command::Subtract{ args } => {
            println!("Result = {}", subtract(args));
        }

        Command::Multiply{ args } => {
            println!("Result = {}", multiply(args));
        }

        Command::Divide{ args } => {
            println!("Result = {}", divide(args));
        }
    }
}
