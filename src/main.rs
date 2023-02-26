use std::io::*;
use text_io::read;


fn interpret(stack: &mut Vec<i32>, line: String) -> Option<String> {
    for word in line.split_whitespace() {
        if let Ok(num) = word.parse() {
            stack.push(num);
        } else {
            match word {
                "dup" => {
                    stack.push(*stack.last()?);
                }
                "+" => {
                    let a = stack.pop()?;
                    let b = stack.pop()?;
                    stack.push(a + b);
                }
                "p" => {
                    println!("{}", stack.last()?);
                }
                _ => {
                    return None
                }
            }
        }
    }
    Some(format!("{:?}", stack))
}


fn main() {
    let mut stack = Vec::<i32>::new();
    loop {
        print!("> ");
        let _ = stdout().flush();
        let line: String = read!("{}\n");
      
        match interpret(&mut stack, line) {
            Some(output) => println!("{}", output),
            None => break,
        }
    }
}
