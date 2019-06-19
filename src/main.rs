use std::env;
use std::collections::HashMap;
use std::process::exit;

type Integer = u32;

trait Computable {
    fn compute(&self) -> Integer {
        return 0;
    }
    fn operator_compute(&self, arg1 : &Computable, arg2: &Computable) -> Integer {
        return 0;
    }
    fn is_operator(&self) -> bool;
}

type Operation = Vec<Box<Computable>>;

struct Constant {
    value : Integer
}

struct  Operator {
    symbol: char
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
        return Box::new(Constant {
            value: to_int(&s)
        });
    } else {
        return Box::new(Operator {
            symbol: first_char
        });
    }
}
//step 1: ^, step 2: * et /, step 3: + et -
fn simplify(op : Operation, step : u8) -> Operation {
    let mut new_op : Operation = Vec::new();
    /*
    done : bool = false;
    while !done {
        for (i, item) in new_op.iter().enumerate() {
            println!("The {}th item is {}", i+1, item);
        }
    }*/
    new_op.push(Box::new(Constant {
        value: 8
    }));
    return new_op;
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
