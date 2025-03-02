use std::io;

fn process_user_input(expression: &str) -> f64 {
    println!("Expression: '{}'", expression.trim());

    let operator = if expression.contains('+') {
        '+'
    } else if expression.contains('-') {
        '-'
    } else if expression.contains('x') {
        'x'
    } else if expression.contains('/') {
        '/'
    } else {
        panic!()
    };

    println!("Operator: {operator}");

    let parts: Vec<&str> = expression.trim().split(operator).collect();
    println!(
        "Split parts: {:?}",
        expression.trim().split(operator).collect::<Vec<&str>>()
    );

    let left = parts.get(0);
    println!("Left side: {:?}", left);
    let left_num = left
        .expect("Failed to unwrap string value!")
        .parse::<f64>()
        .expect("Invalid left number!");

    let right = parts.get(1);
    println!("Right side: {:?}", right);
    let right_num = right
        .expect("Failed to unwrap string value!")
        .parse::<f64>()
        .expect("Invalid left number!");

    match expression.trim() {
        expr if expr.contains('+') => {
            let result = calculate(left_num, right_num, CalcOperations::Add);
            println!("Result: {:?}", result);
            result
        }
        expr if expr.contains('-') => {
            let result = calculate(left_num, right_num, CalcOperations::Subtract);
            println!("Result: {:?}", result);
            result
        }
        expr if expr.contains('x') => {
            let result = calculate(left_num, right_num, CalcOperations::Multiply);
            println!("Result: {:?}", result);
            result
        }
        expr if expr.contains('/') => {
            let result = calculate(left_num, right_num, CalcOperations::Divide);
            println!("Result: {:?}", result);
            result
        }
        _ => {
            panic!("Error")
        }
    }

    // calculate(left, right, operation);
}

fn calculate(left: f64, right: f64, operation: CalcOperations) -> f64 {
    match operation {
        CalcOperations::Add => left + right,
        CalcOperations::Subtract => left - right,
        CalcOperations::Multiply => left * right,
        CalcOperations::Divide => left / right,
    }
}

// fn stringparse(input: String) -> f64 {
// }

enum CalcOperations {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    println!("Welcome to a cli calculator. Enter your expression! ");
    println!("Operations available are: +, -, x, /.");
    println!(
        "Follow the format: '<number><operation><number>', e.g.: 1+5, \
        then hit Enter to see the result"
    );

    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read the provided expression!");

    process_user_input(&expression);
}
