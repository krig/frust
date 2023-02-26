use std::{io::{stdout, Write, BufRead}, collections::HashMap, fs::File, fmt};
use std::fmt::Write as FmtWrite;
use text_io::read;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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

    pub fn pop(&mut self) -> Option<&String> {
        if self.pos < self.stream.len() {
            let ret = self.stream.get(self.pos);
            self.pos += 1;
            ret
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
enum Data {
    Int(i32),
    Str(String),
}

impl Data {
    fn as_int(&self) -> Option<i32> {
        match self {
            Data::Int(x) => Some(*x),
            _ => None,
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Int(i) => write!(f, "{}", i),
            Data::Str(s) => write!(f, "{}", s),
        }
    }
}

struct Vm {
    verbose: bool,
    data: Vec<Data>,
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

    fn interpret(&mut self, pc: Program) -> std::result::Result<String, &'static str> {
        let mut callstack = vec![pc];
        fn popc(cs: &mut Vec<Program>) -> Result<&String, &'static str> {
            let err = "Ran out of program";
            cs.last_mut().ok_or(err)?.pop().ok_or(err)
        }
        while !callstack.is_empty() {
            while let Ok(word) = popc(&mut callstack) {
                if let Ok(num) = word.parse::<i32>() {
                    self.data.push(Data::Int(num));
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
                } else if word == "\"" {
                    let mut s: String = "".to_string();
                    while let Ok(subword) = popc(&mut callstack) {
                        if subword == "\"" {
                            break;
                        } else {
                            match write!(s, " {}", subword).err() {
                                Some(_) => return Err("failed to build string"),
                                None => {},
                            }
                        }
                    }
                    self.data.push(Data::Str(s));
                } else if self.proc.contains_key(word) {
                    let proc = self.proc.get(word).ok_or("Proc not found")?;
                    callstack.push(Program::from_stream(proc.to_vec()));
                } else {
                    match word.as_str() {
                        "dup" => {
                            let val = self.data.last().ok_or("Stack underflow")?.clone();
                            self.data.push(val);
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
                            let a: i32 = self.data.pop().ok_or("Stack underflow")?.as_int().ok_or("Type mismatch")?;
                            let b: i32 = self.data.pop().ok_or("Stack underflow")?.as_int().ok_or("Type mismatch")?;
                            self.data.push(Data::Int(a + b));
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

    fn interpret_line(&mut self, line: String) {
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

    pub fn repl(&mut self) {
        loop {
            print!("> ");
            let _ = stdout().flush();
            let line: String = read!("{}\n");
            self.interpret_line(line);
        }
    }

    pub fn runfile(&mut self, filename: &str) {
        let f = File::open(filename).unwrap();
        for line in std::io::BufReader::new(f).lines() {
            self.interpret_line(line.unwrap());
        }
    }
}


fn main() {
    let args = Args::parse();
    let mut vm = Vm::new(args.verbose);
    if let Some(f) = args.file {
        vm.runfile(f.as_str());
    } else {
        vm.repl();
    }
}
