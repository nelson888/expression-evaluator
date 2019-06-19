use std::env;
use std::process::exit;

type Integer = u32;

trait Computable {
    fn compute(&self) -> Integer {
        return 0;
    }
    fn operator_compute(&self, _arg1 : &Computable, _arg2: &Computable) -> Integer {
        return 0;
    }
    fn is_operator(&self) -> bool;
}

type Operation = Vec<Box<Computable>>;

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

impl Computable for Constant {
    fn compute(&self) -> Integer {
        return self.value;
    }

    fn is_operator(&self) -> bool {
        return false;
    }
}

impl Computable for Operator {
    fn operator_compute(&self, arg1: &Computable, arg2: &Computable) -> Integer {
        let val1 : Integer = arg1.compute();
        let val2 : Integer = arg2.compute();

        return match self.symbol {
            '+' =>  val1 + val2,
            '-' =>  val1 - val2,
            '*' =>  val1 * val2,
            '/' =>  val1 / val2,
            '^' =>  val1 ^ val2,
            _ => {
                println!("Unknown operator {}", self.symbol);
                exit(1);
            }
        }
    }

    fn is_operator(&self) -> bool {
        return true;
    }
}

fn to_int(s: &String) -> Integer {
    return s.parse::<Integer>().unwrap();
}

fn get_first_char(s: &String) -> char {
    return s.chars().next().unwrap();
}

fn to_computable(s: String) -> Box<Computable> {
    let first_char: char = get_first_char(&s);
    if first_char.is_ascii_digit() {
        return Box::new(Constant::of(to_int(&s)));
    } else {
        return Box::new(Operator::of(first_char));
    }
}
//step 1: ^, step 2: * et /, step 3: + et -
fn simplify(mut op : Operation, step : u8) -> Operation {
    loop {
        let opt_op_pos: Option<usize> = op.iter().position(|c| c.is_operator()); //TODO change function is_operator to check if specific operator given the step
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
        println!("TODOOO");
        return;
    }

    let mut step : u8 = 0;
    while operation.len() > 1 {
        operation = simplify(operation, step);
        step += 1;
    }


    println!("{:?}", operation[0].compute());
}
