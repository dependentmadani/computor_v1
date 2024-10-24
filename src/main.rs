use std::env;
use parser::parse;

mod parser;
mod utils;

fn main() {
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);
    if args.len() == 0 {
        println!("Please provide an equation!");
        return;
    } else if args.len() > 1 {
        println!("Please provide the equation inside double quotes, it should be in one argument!");
        return;
    }

    let equation = args[0].clone();
    
    let mut result = match parse::parsing(&equation) {
        Ok(result) => result,
        Err(e) => {
            println!("{}Error:{} {}", red, reset, e);
            return;
        }
    };

    let degree = match utils::utils::print_polynomial_degree(&mut result) {
        Ok(degree) => degree,
        Err(e) => {
            println!("{}Error:{} {}", red, reset, e);
            return;
        }
    };

    utils::utils::print_reduced_form(&result);
    if degree >= 0 && degree <= 2 {
        println!("{}Polynomial degree:{} {}", green, reset, degree);
    } else {
        println!("{}Polynomial degree:{} {}", green, reset, degree);
        println!("{}The polynomial degree is strictly greater than 2, I can't solve.{}", red, reset);
        return;
    }
    match utils::utils::solve_equation(&result, degree) {
        Ok(_) => result,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}
