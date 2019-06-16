use std::env;

type Integer = u32;

struct Constant {
    value : Integer
}

enum Operator {
    PLUS,
    MINUS
}
struct BinaryNode {
    op: Operator,
    arg1 : Computable,
    arg2 : Computable,
}

trait Computable {
    fn compute(&self) -> Integer;
}

impl Computable for Constant {
    fn compute(&self) -> Integer {
        return self.value;
    }
}

impl Computable for Operator {
    fn compute(&self) -> Integer {
        return self.func(self.arg1, self.arg2);
    }
}

fn operate(op: &Operator, arg1 : Integer, arg2 : Integer) -> Integer {
    return match op {
        Operator::PLUS =>  arg1 + arg2,
        Operator::MINUS =>  arg1 - arg2,
    }
}

fn concat(mut a: String, b: String) -> String {
    a.push_str(b.as_str());
    return a;
}

//a=4, b=5; a + a * b
fn main() {
    let operation: String = env::args()
        .skip(1) //skip the name of the program
        .fold(String::new(), concat);



    println!("{:?}", operation);
}
