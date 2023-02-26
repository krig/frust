use std::{io::{stdout, Write, BufRead}, collections::HashMap, fs::File};
use text_io::read;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Interpret file
    #[arg(short, long)]
    file: Option<String>,

    /// Print verbose debug info
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}


struct Program {
    stream: Vec<String>,
    pos: usize,
}

impl Program {
    pub fn from_stream(stream: Vec<String>) -> Self {
        Program {
            stream,
            pos: 0,
        }
    }

    pub fn pop(self: &mut Self) -> Option<&String> {
        if self.pos < self.stream.len() {
            let ret = self.stream.get(self.pos);
            self.pos += 1;
            ret
        } else {
            None
        }
    }
}

struct Vm {
    verbose: bool,
    data: Vec<i32>,
    proc: HashMap<String, Vec<String>>,
}

impl Vm {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            data: vec![],
            proc: HashMap::new(),
        }
    }

    fn interpret(self: &mut Self, pc: Program) -> std::result::Result<String, &'static str> {
        let mut callstack = vec![pc];
        fn popc(cs: &mut Vec<Program>) -> Result<&String, &'static str> {
            cs.last_mut().ok_or("Broken callstack")?.pop().ok_or("Broken callstack")
        }
        while !callstack.is_empty() {
            while let Ok(word) = popc(&mut callstack) {
                if let Ok(num) = word.parse() {
                    self.data.push(num);
                } else if word == ":" {
                    let procname = popc(&mut callstack)?.clone();
                    let mut proc: Vec<String> = vec![];
                    while let Ok(subword) = popc(&mut callstack) {
                        if subword == ";" {
                            break;
                        } else {
                            proc.push(subword.to_string());
                        }
                    }
                    self.proc.insert(procname, proc);
                } else if self.proc.contains_key(word) {
                    let proc = self.proc.get(word).ok_or("Proc not found")?;
                    callstack.push(Program::from_stream(proc.to_vec()));
                } else {
                    match word.as_str() {
                        "dup" => {
                            self.data.push(*self.data.last().ok_or("Nothing to dup")?);
                        }
                        "swap" => {
                            let a = self.data.pop().ok_or("Stack underflow")?;
                            let b = self.data.pop().ok_or("Stack underflow")?;
                            self.data.push(a);
                            self.data.push(b);
                        }
                        "drop" => {
                            self.data.pop().ok_or("Stack underflow")?;
                        }
                        "+" => {
                            let a = self.data.pop().ok_or("Stack underflow")?;
                            let b = self.data.pop().ok_or("Stack underflow")?;
                            self.data.push(a + b);
                        }
                        "print" => {
                            println!("{}", self.data.last().ok_or("Nothing to print")?);
                        }
                        _ => {
                            return Err("Unknown function")
                        }
                    }
                }
            }
            callstack.pop();
        }
        Ok(format!("{:?}", self.data))
    }

    fn interpret_line(self: &mut Self, line: String) {
        let words: Vec<String> = line.split_ascii_whitespace().map(|x| x.to_string()).collect();
        match self.interpret(Program::from_stream(words)) {
            Ok(output) => {
                if self.verbose {
                    println!("{}", output);
                }
            },
            Err(msg) => {
                println!("{}", msg);
            },
        }
    }

    pub fn repl(self: &mut Self) {
        loop {
            print!("> ");
            let _ = stdout().flush();
            let line: String = read!("{}\n");
            self.interpret_line(line);
        }
    }

    pub fn runfile(self: &mut Self, filename: &str) {
        let f = File::open(filename).unwrap();
        for line in std::io::BufReader::new(f).lines() {
            self.interpret_line(line.unwrap());
        }
    }
}


fn main() {
    let args = Args::parse();

    if let Some(f) = args.file {
        Vm::new(args.verbose).runfile(f.as_str());
    } else {
        Vm::new(args.verbose).repl();
    }
}
