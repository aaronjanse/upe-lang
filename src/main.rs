struct Interpreter {
    data: Memory,
    code: Memory,
}

impl Interpreter {
    fn exec(&mut self) -> Vec<Value> {
        while let Some(instr) = self.code.next() {
            match instr {
                Value::u => self.data.get().dec(),
                Value::U => self.data.get().inc(),
                Value::p => self.data.left(),
                Value::P => self.data.right(),
                Value::e => if self.data.get() == &Value::u {
                    let mut depth = 1;
                    while depth > 0 {
                        match self.code.next() {
                            Some(Value::e) => depth += 1,
                            Some(Value::E) => depth -= 1,
                            Some(_) => continue,
                            None => break,
                        }
                    }
                },
                Value::E => if self.data.get() != &Value::u {
                    self.code.left();
                    let mut depth = 1;
                    while depth > 0 {
                        self.code.left();
                        match self.code.get() {
                            Value::e => depth -= 1,
                            Value::E => depth += 1,
                            _ => continue,
                        }
                    }
                },
            }
        }
        self.data.data.clone()
    }
}

#[derive(Debug)]
struct Memory {
    cursor: usize,
    data: Vec<Value>,
}

impl Memory {
    fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    fn right(&mut self) {
        self.cursor += 1;
    }
    fn get(&mut self) -> &mut Value {
        while self.cursor >= self.data.len() {
            self.data.push(Value::u);
        }
        &mut self.data[self.cursor]
    }
    fn next(&mut self) -> Option<Value> {
        let out = self.data.get(self.cursor).copied();
        self.cursor += 1;
        out
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum Value {
    u,
    U,
    p,
    P,
    e,
    E,
}

impl Value {
    fn inc(&mut self) {
        use Value::*;
        *self = match self {
            u => U,
            U => p,
            p => P,
            P => e,
            e => E,
            E => u,
        }
    }

    fn dec(&mut self) {
        use Value::*;
        *self = match self {
            U => u,
            p => U,
            P => p,
            e => P,
            E => e,
            u => E,
        }
    }
}

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    code: String,
    data: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let parse_vec = |s: &str| {
        s.chars().map(|c| {
        use Value::*;
            match c {
                'u' => u,
                'U' => U,
                'p' => p,
                'P' => P,
                'e' => e,
                'E' => E,
                _ => panic!("Character {:?} not in \"upeUPE\"", c),
        }}).collect()
    };
    
    let mut interpreter = Interpreter {
        data: Memory { cursor: 0, data: parse_vec(&args.data.unwrap_or("".to_string())) },
        code: Memory { cursor: 0, data: parse_vec(&args.code) },
    };

    let out_data = interpreter.exec();
    let out_str = out_data.into_iter().map(|v| match v {
        Value::u => "u",
        Value::U => "U",
        Value::p => "p",
        Value::P => "P",
        Value::e => "e",
        Value::E => "E",
    }).collect::<Vec<_>>().join("");

    println!("{out_str}");
}
