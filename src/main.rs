use std::{env, io};
use std::process::exit;
use std::collections::HashMap;
use std::io::Stdin;

const PLUS : char = '+';
const MINUS : char = '-';
const DIVIDE : char = '/';
const MULTIPLY : char = 'x';
const MULTIPLY_2 : char = '*';
const POWER : char = '^';

const OPERATORS : [char; 6] = [PLUS, MINUS, DIVIDE, MULTIPLY, MULTIPLY_2, POWER];

const POWER_STEP : u8 = 0;
const MULTIPLY_STEP : u8 = 1;
const ADD_STEP : u8 = 2;

fn last_step() -> u8 {
    return ADD_STEP;
}

type Integer = i32;

fn pow(a: Integer, n : Integer) -> Integer {
    if n == 0 {
        return 1;
    }
    if a == 0 {
        return 0;
    }
    if n % 2 == 0 {
        let p = pow(a, n / 2);
        return p * p;
    }
    return a * pow(a, n - 1);
}

trait Computable {
    fn compute(&self) -> Integer {
        return 0;
    }
    fn operator_compute(&self, _arg1 : &Computable, _arg2: &Computable) -> Integer {
        return 0;
    }
    fn is_operator(&self, _c : char) -> bool {
        return false;
    }
}

struct Constant {
    value : Integer
}
impl Constant {
    fn of(value : Integer) -> Constant {
        return Constant {
            value
        };
    }
}
impl Computable for Constant {
    fn compute(&self) -> Integer {
        return self.value;
    }
}

struct Operator {
    symbol: char
}
impl Operator {
    fn of(symbol : char) -> Operator {
        return Operator {
            symbol
        };
    }
}
impl Computable for Operator {
    fn operator_compute(&self, arg1: &Computable, arg2: &Computable) -> Integer {
        let val1 : Integer = arg1.compute();
        let val2 : Integer = arg2.compute();

        return match self.symbol {
            PLUS =>  val1 + val2,
            MINUS =>  val1 - val2,
            MULTIPLY |  MULTIPLY_2 =>  val1 * val2,
            DIVIDE =>  val1 / val2,
            POWER =>  pow(val1, val2),
            _ => {
                println!("Unknown operator {}", self.symbol);
                exit(1);
            }
        }
    }

    fn is_operator(&self, c: char) -> bool {
        return self.symbol == c;
    }
}

fn to_int(s: &str) -> Integer {
    return s.parse::<Integer>().unwrap();
}

fn get_first_char(s: &str) -> char {
    return s.chars().next().unwrap();
}

fn step_operator_finder(c: &Box<Computable>, step: u8) -> bool {
    return match step {
        POWER_STEP => c.is_operator(POWER),
        MULTIPLY_STEP => c.is_operator(MULTIPLY) || c.is_operator(MULTIPLY_2)
            || c.is_operator(DIVIDE),
        ADD_STEP => c.is_operator(PLUS) || c.is_operator(MINUS),
        _ => false
    }
}

fn is_unary_operator(c : char) -> bool {
    return c == MINUS;
}

fn evaluate_unary(c: char, n: Integer) -> Integer {
    return match c {
        MINUS => - n,
        _ => {
            println!("Unknown unary operator {}", c);
            exit(1);
        }
    }
}

fn to_computable(s: &str, opt_variable_map: Option<&HashMap<String, Integer>>) -> Box<Computable> {
    let first_char: char = get_first_char(&s);
    if OPERATORS.contains(&first_char) && s.len() == 1 {
        return Box::new(Operator::of(first_char));
    } else if is_unary_operator(first_char) {
        let num_string : String = s.chars().skip(1).collect();
        return Box::new(Constant::of(evaluate_unary(first_char,
                                                    to_int(&num_string))));
    } else {
        if first_char.is_ascii_digit() {
            return Box::new(Constant::of(to_int(&s)));
        } else if opt_variable_map.is_some() {
            let opt_value:Option<&Integer> = opt_variable_map.unwrap().get(&String::from(s));
            if opt_value.is_none() {
                println!("'{}' is not defined", s);
                exit(1);
            }
            return Box::new(Constant::of(opt_value.unwrap().clone()));
        } else {
            println!("'{}' is not a number", s);
            exit(1);
        }
    }
}

type Expression = Vec<Box<Computable>>;
//step 1: ^, step 2: * et /, step 3: + et -
fn evaluate_step(mut expr : Expression, step : u8) -> Expression {
    loop {
        let opt_op_pos: Option<usize> = expr.iter()
            .position(|c| step_operator_finder(c, step));
        if opt_op_pos.is_none() {
            break;
        }
        //get the operator and compute
        let operator_pos: usize = opt_op_pos.unwrap();
        let result = Box::new(
            Constant::of(
                expr[operator_pos].as_ref()
                    .operator_compute(expr[operator_pos - 1].as_ref(),
                                      expr[operator_pos + 1].as_ref())
        ));

        //replace the operator and its parameter to the result
        expr.remove(operator_pos);
        expr.insert(operator_pos, result);
        expr.remove(operator_pos + 1);
        expr.remove(operator_pos - 1);
    }
    return expr;
}

fn evaluate(mut expression : Expression) -> Integer {
    let mut step : u8 = 0;
    while expression.len() > 1 && step <= last_step() {
        expression = evaluate_step(expression, step);
        step += 1;
    }
    return expression[0].compute();
}

fn main() {
    let expression: Expression = env::args()
        .skip(1) //skip the name of the program
        .map(|s| to_computable(s.as_str(), None))
        .collect();

    if expression.len() == 0 {
        scan_operation();
        return;
    }
    println!("{:?}", evaluate(expression));
}

fn scan_operation() {
    let mut variable_map: HashMap<String, Integer> = HashMap::new();

    let stdin : Stdin = io::stdin();
    let mut line: String = String::new();
    loop {
        line.truncate(0);
        stdin.read_line(&mut line);

        if !line.trim().contains("=") {
            println!("{:?}", evaluate(line.split_whitespace()
                .map(|s| to_computable(s, Some(&variable_map)))
                .collect()));
            break;
        }

        let assignment : Vec<String> = line.split("=").map(|s| String::from(s)).collect();
        if assignment.len() != 2 {
            print!("{} is not properly formatted (should be 'variable = expression)'", line);
            return;
        }
        let identifier = String::from(assignment[0].trim());
        let expression : Vec<Box<Computable>> = assignment[1].split_whitespace()
            .map(|s| to_computable(s, Some(&variable_map)))
            .collect();
        variable_map.insert(String::from(identifier), evaluate(expression));
    }
}