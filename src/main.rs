use text_io::read;
use std::io::*;



fn interpret(stack: &mut Vec<i32>, line: String) {
    for word in line.split_whitespace() {
        if let Ok(num) = word.parse() {
            stack.push(num);
        } else {
            match word {
                "dup" => {
                    stack.push(*stack.last().unwrap());
                }
                "+" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                }
                "p" => {
                    println!("{}", stack.last().unwrap());
                }
                _ => {
                }
            }
        }
        println!("{:?}", stack);
    }
}

fn main() {
    let mut stack = Vec::<i32>::new();
    loop {
        print!("> ");
        let _ = stdout().flush();
        let line: String = read!("{}\n");
      
        interpret(&mut stack, line);
    }
}
