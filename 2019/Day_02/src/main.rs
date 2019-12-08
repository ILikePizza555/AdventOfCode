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

    fn read_location(&self, i: usize) -> i32 {
        return self.memory[self.memory[i] as usize];
    }

    fn step(self) -> Result<MachineState, String> {
        if self.halt == true {
            return Err("Machine is halted.".to_string());
        }

        let opcode = self.memory[self.instruction_pointer];

        if opcode == 1 {
            // Addition
            let a = self.read_location(self.instruction_pointer + 1);
            let b = self.read_location(self.instruction_pointer + 2);
            let loc = self.memory[self.instruction_pointer + 3] as usize;
            let next_pointer = self.instruction_pointer + 4;

            return self.write_memory(loc, a + b).and_then(|m| m.set_instruction_pointer(next_pointer));
        }
        else if opcode == 2 {
            // Multiplication
            let a = self.read_location(self.instruction_pointer + 1);
            let b = self.read_location(self.instruction_pointer + 2);
            let loc = self.memory[self.instruction_pointer + 3] as usize;
            let next_pointer = self.instruction_pointer + 4;

            return self.write_memory(loc, a * b).and_then(|m| m.set_instruction_pointer(next_pointer));
        }
        else if opcode == 99 {
            return Ok(self.halt());
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
    let mut machine_state = make_machine_state(get_input_code())
        .write_memory(1, 12)
        .and_then(|m| m.write_memory(2, 2))
        .expect("Error: ");

    while machine_state.halt != true {
        machine_state = machine_state.step().expect("Error: ");
    }
    
    println!("Memory: {:?}", machine_state.memory);
}
