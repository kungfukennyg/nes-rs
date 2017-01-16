use super::super::std::string::String;
use super::cpu::Cpu;
use std::fmt;
use std::collections::HashMap;

pub struct InstructionTable<'a> {
    instructions: HashMap<u8, Instruction<'a>>
}

impl<'a> InstructionTable<'a> {
    pub fn new() -> Self {
        let mut instruction_table = InstructionTable {
            instructions: HashMap::new()
        };

        instruction_table.init_instructions();

        instruction_table
    }

    pub fn exec_instruction(&mut self, opcode: u8, cpu: &Cpu) {
        self.instructions.get_mut(&opcode).unwrap().execute(cpu);
    }

    fn init_instructions(&mut self) {
        // LDA
        for &o in vec![0xa1, 0xa5, 0xa9, 0xad, 0xb1, 0xb5, 0xb9, 0xbd].iter() {
            let opcode = o as u8;

            let mut instruction = Instruction::new("LDA".to_string(), opcode);

            instruction.set(move |cpu| {
                println!("test LDA");
                1
            });

            self.instructions.insert(opcode, instruction);
        }
    }
}

impl<'a> fmt::Debug for InstructionTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Todo: impl debug for instruction table")
    }
}

pub struct Instruction<'a> {
    name: String,
    opcode: u8,
    function: Option<Box<FnMut(&Cpu) -> u8 + 'a>>
}

impl<'a> Instruction<'a> {
    fn new(name: String, opcode: u8) -> Instruction<'a> {
        Instruction {
            name: name,
            opcode: opcode,
            function: None
        }
    }

    fn set<T: FnMut(&Cpu) -> u8 + 'a>(&mut self, f: T) {
        self.function = Some(Box::new(f));
    }

    pub fn execute(&mut self, cpu: &Cpu) -> u8 {
        match self.function {
            Some(ref mut function) => (function)(cpu),
            None => 0
        }
    }
}