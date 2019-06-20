use std::env;
use std::process::exit;

const PLUS : char = '+';
const MINUS : char = '-';
const DIVIDE : char = '/';
const MULTIPLY : char = 'x';
const POWER : char = '^';

const POWER_STEP : u8 = 0;
const MULTIPLY_STEP : u8 = 1;
const ADD_STEP : u8 = 2;

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
            MULTIPLY =>  val1 * val2,
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

fn to_int(s: &String) -> Integer {
    return s.parse::<Integer>().unwrap();
}

fn get_first_char(s: &String) -> char {
    return s.chars().next().unwrap();
}

fn step_operator_finder(c: &Box<Computable>, step: u8) -> bool {
    return match step {
        POWER_STEP => c.is_operator(POWER),
        MULTIPLY_STEP => c.is_operator(MULTIPLY) || c.is_operator(DIVIDE),
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

fn to_computable(s: String) -> Box<Computable> {
    let first_char: char = get_first_char(&s);
    if first_char.is_ascii_digit() {
        return Box::new(Constant::of(to_int(&s)));
    } else if is_unary_operator(first_char) {
        let num_string : String = s.chars().skip(1).collect();
        return Box::new(Constant::of(evaluate_unary(first_char,
                                                    to_int(&num_string))));
    } else {
        return Box::new(Operator::of(first_char));
    }
}

type Operation = Vec<Box<Computable>>;
//step 1: ^, step 2: * et /, step 3: + et -
fn compute(mut op : Operation, step : u8) -> Operation {
    loop {
        let opt_op_pos: Option<usize> = op.iter()
            .position(|c| step_operator_finder(c, step));
        if opt_op_pos.is_none() {
            break;
        }
        //get the operator and compute
        let operator_pos: usize = opt_op_pos.unwrap();
        let result = Box::new(
            Constant::of(
                op[operator_pos].as_ref()
                    .operator_compute(op[operator_pos - 1].as_ref(),
                                      op[operator_pos + 1].as_ref())
        ));

        //replace the operator and its parameter to the result
        op.remove(operator_pos);
        op.insert(operator_pos, result);
        op.remove(operator_pos + 1);
        op.remove(operator_pos - 1);
    }
    return op;
}
//a=4, b=5; a + a * b
fn main() {
    let mut operation: Operation = env::args()
        .skip(1) //skip the name of the program
        .map(|s| s) //TODO traiter variables
        .map(to_computable)
        .collect();

    if operation.len() == 0 {
        //TODO scanf operations???
        println!("TODOOO handle when no argument");
        return;
    }

    let mut step : u8 = 0;
    while operation.len() > 1 && step <= ADD_STEP {
        operation = compute(operation, step);
        step += 1;
    }

    println!("{:?}", operation[0].compute());
}
