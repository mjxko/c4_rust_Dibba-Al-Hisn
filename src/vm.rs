// This is a simple virtual machine.
// It runs instructions and uses a stack to do math.
pub struct VM {
    pub instructions: Vec<String>, // List of instructions (like ADD, IMM 5, etc.)
    pub stack: Vec<i64>,           // Stack to store numbers while doing calculations
}

impl VM {
    // Makes a new VM with some instructions and an empty stack
    pub fn new(instructions: Vec<String>) -> Self {
        VM {
            instructions,
            stack: Vec::new(),
        }
    }

    // This runs the instructions one by one
    pub fn run(&mut self) {
        for inst in &self.instructions {
            match inst.as_str() {
                // Add the top two numbers from the stack
                "ADD" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }

                // Subtract the top number from the second top number
                "SUB" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }

                // Multiply the top two numbers
                "MUL" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }

                // Divide the second top number by the top number
                "DIV" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }

                // Get the remainder after division
                "MOD" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a % b);
                }

                // Print the top number on the stack
                "PRTF" => {
                    let value = self.stack.pop().unwrap();
                    println!("{}", value);
                }

                // Push a number to the stack (like IMM 5 pushes 5)
                s if s.starts_with("IMM") => {
                    let value: i64 = s[4..].trim().parse().unwrap();
                    self.stack.push(value);
                }

                // "LEV" means "leave" — stop running instructions
                "LEV" => {
                    break;
                }

                // If the instruction is unknown, show an error
                _ => panic!("Unknown instruction: {}", inst),
            }
        }
    }
}
