extern crate num;

use num::Integer;
use std::error::Error;

struct MachineState
{
    halt: bool,
    memory: Vec<i32>,
    instruction_pointer: usize
}

impl MachineState {
    fn set_instruction_pointer(self, to: usize) -> Result<MachineState, String> {
        if to > self.memory.len() {
            return Err(format!("New instruction pointer {} outside of memory range {}", to, self.memory.len()));
        }
        
        return Ok(MachineState {
            halt: self.halt,
            memory: self.memory,
            instruction_pointer: to
        })
    }

    fn write_memory(self, i: usize, d: i32) -> Result<MachineState, String> {
        if i > self.memory.len() {
            return Err(format!("Write to {} outside of memory range {}", i, self.memory.len()))
        }

        let mut rv = MachineState {
            halt: self.halt,
            memory: self.memory,
            instruction_pointer: self.instruction_pointer
        };
        rv.memory[i] = d;
        return Ok(rv);
    }
    
    fn halt(self) -> MachineState {
        return MachineState {
            halt: true,
            memory: self.memory,
            instruction_pointer: self.instruction_pointer
     
        }
    }

    fn read_memory(&self, i: usize, n: usize) -> &[i32] {
        return &self.memory[i..i + n];
    }

    fn step(self) -> Result<MachineState, String> {
        if self.halt == true {
            return Err("Machine is halted.".to_string());
        }

        let opcode = self.memory[self.instruction_pointer];
        let a = self.memory[self.instruction_pointer + 1];
        let b = self.memory[self.instruction_pointer + 2];
        let loc = self.memory[self.instruction_pointer + 3] as usize;

        let mut new_state = MachineState {
            halt: self.halt,
            memory: self.memory,
            instruction_pointer: self.instruction_pointer + 4
        };

        if opcode == 1 {
            // Addition
            new_state.memory[loc] = a + b;
            
            return Ok(new_state);
        }
        else if opcode == 2 {
            // Multiplication
            new_state.memory[loc] = a * b;

            return Ok(new_state);
        }
        else if opcode == 99 {
            return Ok(new_state.halt());
        }
        else {
            return Err(format!("Invalid opcode {} at memory location {}", opcode, self.instruction_pointer));
        }
    }
}

fn make_machine_state(memory: Vec<i32>) -> MachineState {
    return MachineState {
        halt: false,
        memory: memory,
        instruction_pointer: 0
    }
}

fn get_input_code() -> Vec<i32> {
    return include_str!("input1.csv")
        .trim()
        .split(",")
        .map(|x| str::parse::<i32>(x).unwrap())
        .collect::<Vec::<i32>>();
}

fn main() {
    let mut machine_state = make_machine_state(vec![1,0,0,0,99])
        .write_memory(1, 12)
        .and_then(|m| m.write_memory(2, 2))
        .expect("Error: ");

    while machine_state.halt != true {
        machine_state = machine_state.step().expect("Error: ");
    }
    
    println!("Memory: {:?}", machine_state.memory);
}
