use std::collections::HashMap;
use std::io;

fn check_allowed_characters(equation: &String) -> bool {
    let numbers = "0123456789";
    let allowed = "X^-.* ";
    let mut legit = true;

    for c in equation.chars() {
        if !numbers.contains(c) && !allowed.contains(c) {
            legit = false;
            break;
        }
    }

    legit
}

fn check_form(part_equation: &String) -> bool {
    let numbers = "0123456789";
    let mut number = false;
    let mut decimal = false;
    let mut sign = false;
    let mut x = false;
    let mut sign_2 = false;
    let mut power = false;
    let mut space = false;

    for (i, c) in part_equation.chars().enumerate() {
        if (numbers.contains(c) || c == '-') && !number && i == 0 {
            number = true;
            if c == '-' {
                sign = true;
            }
        } else if c == '.' && number && !decimal {
            decimal = true;
        } else if c == '*' && (number || sign) {
            sign = true;
            decimal = false; // Reset decimal for next number
        } else if c == 'X' && (sign || number) {
            x = true;
        } else if c == '^' && x && !space {
            sign_2 = true;
        } else if numbers.contains(c) && sign_2 && !space {
            power = true;
        } else if c == ' ' && x {
            space = true;
        }
    }

    if (number || sign) && x && (!sign_2 && power || sign_2 && !power) {
        return true;
    } else if (number && !x) || (x && !sign) {
        return true;
    }
    false
}

fn solve_equation_degree_1(result: &HashMap<String, f32>) -> i32 {
    let mut a = 0.0;
    let mut b = 0.0;

    for (key, value) in result {
        if key.contains("X^1") || key == "X" {
            a = *value;
        } else if key.contains("X^0") {
            b = *value;
        }
    }
    if a == 0.0 {
        return -1;
    }
    let x = -b / a;
    println!("The solution is:\n{}", x);
    return 0;
}

fn solve_equation_degree_2(result: &HashMap<String, f32>) -> i32 {
    let green = "\x1b[32m";
    let reset = "\x1b[0m";
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;

    for (key, value) in result {
        if key.contains("X^2") {
            a = *value;
        } else if key.contains("X^0") || !key.contains("X") {
            c = *value;
        } else if key.contains("X^1")  || key.contains("X") {
            b = *value;
        }
    }

    if a == 0.0 {
        return -1;
    }
    let delta = b * b - 4.0 * a * c;
    println!("First, we need to calculate the discriminant:\n delta = b^2 - 4 *a *c = {}^2 - 4 * {} * {} = {}", b, a, c, delta);
    if delta > 0.0 {
        println!("Discriminant is strictly positive, we will move on to calculate the two solutions.");
        println!("The first solution will be in the form of:\n x1 = (-b + √delta) / 2a\n\t= (-{} + √{}) / 2 * {}\n\t = (-{} + {}) / {}", b, delta, a, b, delta.sqrt(), 2.0 * a);
        let x1 = (-b + delta.sqrt()) / (2.0 * a);
        println!("The second solution will be in the form of:\n x2 = (-b - √delta) / 2a\n\t = (-{} - √{}) / 2 * {}\n\t = (-{} - {}) / {}", b, delta, a, b, delta.sqrt(), 2.0 * a);
        let x2 = (-b - delta.sqrt()) / (2.0 * a);
        println!("{}Finally, the two solutions are:{}\n{:.2} and {:.2}", green, reset, x1, x2);
    } else if delta == 0.0 {
        println!("Discriminant is equal to zero, we will move on to calculate the solution.");
        println!("The solution will be in the form of:\n x = -b / 2a = -{} / 2 * {} = -{} / {}", b, a, b, 2.0 * a);
        let x = -b / (2.0 * a);
        println!("{}Finally, the solution is:{}\n{}", green, reset, x);
    } else {
        println!("Discriminant is strictly negative, we will move on to calculate the two solutions.");
        println!("The two solutions will be in the form of:\n x1 = (-b + i * √|delta|) / 2a\n\t = (-{} + i * √{}) / 2 * {}\n\t = (-{} + i * {}) / {}", b, delta * -1.0, a, b, (delta * -1.0).sqrt(), 2.0 * a);
        let real = -b / (2.0 * a);
        let imaginary = (delta * -1.0).sqrt() / (2.0 * a);
        println!("{}Finally, the two solutions are:{}\n{:.2} + i * {:.2} and {:.2} - i * {:.2}", green, reset, real, imaginary, real, imaginary);
    }
    return 0;
}

pub fn valid_equation_equal_operator(s: &String) -> bool {
    let mut equal_operator = 0;
    
    for c in s.chars() {
        if c == '=' {
            equal_operator += 1;
        }
        if equal_operator > 1 {
            return false;
        }
    }
    if equal_operator == 0 {
        return false;
    }
    true
}

pub fn is_valid_number(s: &str) -> bool {
    let mut chars = s.chars().peekable();
    let mut has_digit = false;
    let mut has_decimal = false;

    if let Some(&c) = chars.peek() {
        if c == '-' || c == '+' {
            chars.next();
        }
    }

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            has_digit = true;
        } else if c == '.' {
            if has_decimal || !has_digit {
                return false;
            }
            has_decimal = true;
            has_digit = false;
        } else {
            return false;
        }
    }

    has_digit
}

pub fn check_equation_format(equation: &String) -> bool {

    if !check_allowed_characters(equation) {
        return false;
    }

    if !equation.contains("^") && !equation.contains("X") {
        return is_valid_number(&equation);
    }

    if check_form(equation) {
        return false;
    }

    true
}

pub fn print_reduced_form(result: &HashMap<String, f32>) {
    let green = "\x1b[32m";
    let reset = "\x1b[0m";
    let mut reduced_form = String::new();
    let mut first = true;

    let mut normalized_result = HashMap::new();

    // Normalize keys
    for (key, value) in result.iter() {
        let normalized_key = if key == "X" || key == "X^1" {
            "X".to_string()
        } else if key == "X^0" {
            "".to_string()
        } else if key == "-X^0" {
            "".to_string()
        } else if key.contains("X^0") {
            key.replace("X^0", "")
        } else {
            key.clone()
        };

        *normalized_result.entry(normalized_key).or_insert(0.0) += value;
    }

    let mut keys: Vec<&String> = normalized_result.keys().collect();
    keys.sort_by(|a, b| b.cmp(a));

    for key in keys {
        if let Some(value) = normalized_result.get(key) {
            if *value == 0.0 {
                continue;
            }

            if *value > 0.0 && !first {
                reduced_form.push_str(" + ");
            } else if *value < 0.0 {
                reduced_form.push_str(" - ");
            }

            if *value != 1.0 && *value != -1.0 {
                let val = if *value < 0.0 { -*value } else { *value };
                if val.fract() == 0.0 {
                    let val_int = val as i32;
                    reduced_form.push_str(&val_int.to_string());
                } else {
                    reduced_form.push_str(&val.to_string());
                }
                if !key.is_empty() {
                    reduced_form.push_str(" * ");
                }
            } else if *value == -1.0 && !key.is_empty() {
                reduced_form.push_str("1 * ");
            } else if (*value == 1.0 || *value == -1.0 ) && key.is_empty() {
                reduced_form.push_str("1");
            }

            if !key.is_empty() {
                reduced_form.push_str(key);
            }
            first = false;
        }
    }
    if first {
        reduced_form.push_str("0");
    }
    reduced_form.push_str(" = 0");

    println!("{}Reduced form:{} {}", green, reset, reduced_form);
}

pub fn print_polynomial_degree(result: &HashMap<String, f32>) -> Result<i32, io::Error> {
    let mut degree: i32 = 0;

    for (key, _) in result {
        let mut temp = 0;
        if key.contains("X") {
            let mut split = 1;
            if key.contains("^") {
                split = key.split('^').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            }
            temp = split;
        }
        if temp > degree {
            degree = temp;
        }
    }

    return Ok(degree);
}

pub fn solve_equation(result: &HashMap<String, f32>, degree: i32) -> Result<i32, io::Error> {
    if degree == 0 {
        if result.contains_key("X^0") {
            if result.get("X^0").unwrap() == &0.0 {
                println!("All real numbers are solutions.");
            } else {
                println!("There is no solution.");
            }
        } else {
            println!("There is no solution.");
        }
    } else if degree == 1 {
        let output = solve_equation_degree_1(result);
        if output == -1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "There is no solution!",
            ));
        }
    } else if degree == 2 {
        let output = solve_equation_degree_2(result);
        if output == -1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "There is no solution!",
            ))
        }
    }
    return Ok(0);
}