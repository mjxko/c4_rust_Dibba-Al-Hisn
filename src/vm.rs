// vm.rs

pub struct VM {
    pub instructions: Vec<String>,
    pub stack: Vec<i64>,
}

impl VM {
    pub fn new(instructions: Vec<String>) -> Self {
        VM {
            instructions,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        for inst in &self.instructions {
            match inst.as_str() {
                "ADD" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                "SUB" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                "MUL" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                "DIV" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                "MOD" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a % b);
                }
                "PRTF" => {
                    let value = self.stack.pop().unwrap();
                    println!("{}", value);
                }
                s if s.starts_with("IMM") => {
                    let value: i64 = s[4..].trim().parse().unwrap();
                    self.stack.push(value);
                }
                "LEV" => {
                    break; // simulate function return
                }
                _ => panic!("Unknown instruction: {}", inst),
            }
        }
    }
}
