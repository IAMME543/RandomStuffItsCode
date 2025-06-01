use std::collections::HashSet;
use std::f64::consts::PI;
use std::io;

fn main() {
    let mut input: String = "".to_string();
    io::stdin().read_line(&mut input).expect("Error");

    let tokens = string_to_clean_terms(input);

    let tokens_str: Vec<&str> = tokens.iter().map(|s| s.trim()).collect();

    let postfix = shunting_yard(tokens_str);

    math(postfix);

    println!("Press Enter To End Your Session");

    let mut exit: String = "".to_string();
    io::stdin()
        .read_line(&mut exit)
        .expect("If you got this error then you broke it");
}

fn string_to_clean_terms(data: String) -> Vec<String> {
    let vaild_operators = HashSet::from(['+', '-', '/', '*', '%', '(', ')', 's']);

    let tokens: String = data
        .chars()
        .filter(|c| {
            !c.is_whitespace() && vaild_operators.contains(c)
                || *c == '.'
                || *c == 'p'
                || *c == 'P'
                || c.is_numeric()
        })
        .collect();

    let mut filtered: Vec<String> = Vec::new();
    let mut last_operator_index: i32 = -1;

    for (i, c) in tokens.chars().enumerate() {
        if vaild_operators.contains(&c) {
            if last_operator_index + 1 != i as i32 {
                filtered.push(tokens[(last_operator_index + 1) as usize..i].to_string());
            }

            filtered.push(c.to_string());

            last_operator_index = i as i32;
        }
        if i + 1 == tokens.len() {
            filtered.push(tokens[i..].to_string());
        }
    }

    //constants replacement

    let new_filtered: Vec<String> = filtered
        .iter()
        .map(|c| {
            if *c == 'p'.to_string() || *c == 'P'.to_string() {
                PI.to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<String>>();

    for new in &filtered {
        print! {"{}: ", new};
    }

    return new_filtered;
}

fn math(tokens: Vec<String>) {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token.parse::<f64>() {
            Ok(num) => {
                stack.push(num);
            }

            Err(_) => {
                if stack.len() < 1 {
                    println!("Nothing has been pushed to stack");
                    return;
                }
                let operand_2 = stack.pop().unwrap();

                let mut operand_1: f64 = 0.0;
                if token != "s" {
                    operand_1 = stack.pop().unwrap()
                }

                let result = match token.as_str() {
                    "+" => operand_1 + operand_2,
                    "-" => operand_1 - operand_2,
                    "*" => operand_1 * operand_2,
                    "/" => {
                        if operand_2 == 0.0 {
                            eprintln!("Divide By Zero Error");
                            return;
                        }
                        operand_1 / operand_2
                    }
                    "s" => operand_2.sqrt(),
                    _ => {
                        eprintln!("No Operator Was Found");
                        return;
                    }
                };

                stack.push(result);
            }
        }
    }
    println!("= {}", stack.last().unwrap());
}

fn shunting_yard(tokens: Vec<&str>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    for token in tokens {
        if let Ok(_) = token.parse::<f64>() {
            output.push(token.to_string());
        } else if token == "(" {
            operators.push(token);
        } else if token == ")" {
            while *operators.last().unwrap() != "(" {
                output.push(operators.last().unwrap().to_string());
                operators.pop();
            }
            if *operators.last().unwrap() == "(" {
                operators.pop();
            }
        } else {
            while let Some(&top) = operators.last() {
                if precedence(top) >= precedence(token) {
                    output.push(top.to_string());
                    operators.pop();
                } else {
                    break;
                }
            }
            operators.push(token);
        }
    }

    while let Some(&top) = operators.last() {
        output.push(top.to_string());
        operators.pop();
    }

    output
}

fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" | "%" => 2,
        "s" => 3,
        _ => 0,
    }
}
