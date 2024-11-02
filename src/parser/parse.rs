use std::collections::HashMap;
use std::io;
use utils::utils;

fn check_allowed_characters_equation(equation: &String) -> bool {
    let numbers = "0123456789";
    let allowed = "X^+-.*= ";
    let mut legit = true;

    for c in equation.chars() {
        if !numbers.contains(c) && !allowed.contains(c) {
            legit = false;
            break;
        }
    }

    legit
}

fn split_with_signs(input: &String) -> Result<Vec<String>, io::Error> {
    let numbers = "0123456789";
    let mut result = Vec::new();
    let mut temp = String::new();
    let mut all_signs = false;
    let mut number = false;
    for c in input.chars() {
        if (c == '+' || c == '-' || c == '*') && all_signs {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Combined signs are not authorized in the format equation!",
            ));
        }
        if c == '*' && !number {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Missing value before the * sign!",
            ));
        }
        if c == '+' || c == '-' {
            if temp.len() > 0 {
                result.push(temp.trim().to_string());
                temp.clear();
            }
            all_signs = true;
            number = false;
            if c == '-' {
                temp.push(c);
            }
        } else if numbers.contains(c) || c == '.' {
            number = true;
            all_signs = false;
            temp.push(c);
        } else if number {
            temp.push(c);
        } else if c == 'X' || c == '^' {
            all_signs = false;
            temp.push(c);
        }
    }
    if temp.len() > 0 {
        result.push(temp.trim().to_string());
        temp.clear();
    }

    Ok(result)
}

fn check_x_side(part: &String) -> bool {
    let allowed = "X^0123456789-";
    let numbers = "0123456789";
    let mut counter = 0;
    let mut sign = 0;
    let mut negative_sign = false;
    let mut number = false;

    if !part.contains("X") {
        return true;
    }
    if part.len() == 1 && part.contains("X") {
        return true;
    }
    if part.chars().nth(0).unwrap() == '-' {
        negative_sign = true;
    }

    for c in part.chars() {
        if (counter == 0 && c != 'X' && !negative_sign)
            || (negative_sign && counter == 1 && c != 'X')
        {
            return false;
        } else if counter != 0 && c == 'X' && !negative_sign {
            return false;
        }
        if counter > 1 {
            negative_sign = false;
        }
        if c == '^' {
            sign += 1;
            if sign > 1 {
                return false;
            }
        }
        if !allowed.contains(c) {
            return false;
        }
        if numbers.contains(c) && sign > 0 && counter > 1 {
            number = true;
        }
        counter += 1;
    }
    if !number && sign > 0 {
        return false;
    }
    true
}

fn fill_map(info: &mut HashMap<String, f32>, part: &String, side: &str) -> Result<(), io::Error> {
    if part.len() == 1 && part.contains("0") {
        return Ok(());
    }

    let mut left_split_part = "1";
    let mut right_split_part = "";

    //Check left side
    if part.contains("*") {
        left_split_part = part.split('*').collect::<Vec<&str>>()[0].trim();
        if !utils::is_valid_number(&left_split_part) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid format of the value!",
            ));
        };
    } else if !part.contains("X") {
        left_split_part = part.trim();
        right_split_part = "X^0";
    } else if part.trim().chars().nth(0).unwrap() == '-' {
        left_split_part = "-1";
    }

    //Check right side
    if part.contains("*") {
        right_split_part = part.split('*').collect::<Vec<&str>>()[1].trim();
        if !check_x_side(&right_split_part.to_string()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid format on the X side!",
            ));
        }
    } else if part.contains("X") {
        right_split_part = part.trim();
        if !check_x_side(&right_split_part.to_string()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid format on the X side!",
            ));
        }
    } else if right_split_part.len() != 0 {
        if !check_x_side(&right_split_part.to_string()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid format on the X side!",
            ));
        }
    }
    let number = left_split_part.parse::<f32>().unwrap();

    if info.contains_key(right_split_part) {
        if side == "left" {
            *info.get_mut(right_split_part).unwrap() += number;
        } else {
            *info.get_mut(right_split_part).unwrap() -= number;
        }
    } else {
        if side == "left" {
            info.insert(right_split_part.to_string(), number);
        } else {
            info.insert(right_split_part.to_string(), (-1.0) * number);
        }
    }
    Ok(())
}

pub fn parsing(input: &String) -> Result<HashMap<String, f32>, io::Error> {
    let mut info = HashMap::new();

    if !check_allowed_characters_equation(input) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The provided equation contains unknown characters!",
        ))
    }

    if !utils::valid_equation_equal_operator(input) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The input should be a valid equation with 1 equal operator!",
        ))
    }

    let left = input.split('=').collect::<Vec<&str>>()[0].trim();
    let right = input.split('=').collect::<Vec<&str>>()[1].trim();

    if left.len() == 0 || right.len() == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid format of the equation, an empty side of equation!",
        ));
    }

    let left_split = match split_with_signs(&left.to_string()) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };
    let right_split = match split_with_signs(&right.to_string()) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };
    
    for part in left_split {
        match utils::check_equation_format(&part.to_string())  {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match fill_map(&mut info, &part.to_string(), "left") {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    for part in right_split {
        match utils::check_equation_format(&part.to_string())  {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match fill_map(&mut info, &part.to_string(), "right") {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    Ok(info)
}
