extern crate num;

use num::Integer;
use std::error::Error;

struct MachineState<TMem: Integer>
{
    halt: bool,
    memory: Vec<TMem>,
    instruction_pointer: usize
}

impl<TMem: Integer> MachineState<TMem> {
    fn set_instruction_pointer(self, to: usize) -> Result<MachineState<TMem>, String> {
        if to > self.memory.len() {
            return Err(format!("New instruction pointer {} outside of memory range {}", to, self.memory.len()));
        }
        
        return Ok(MachineState {
            halt: self.halt,
            memory: self.memory,
            instruction_pointer: to
        })
    }

    fn write_memory(self, i: usize, d: TMem) -> Result<MachineState<TMem>, String> {
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
    
    fn halt(self) -> MachineState<TMem> {
        return MachineState {
            halt: true,
            memory: self.memory,
            instruction_pointer: self.instruction_pointer
        }
    }

    fn read_memory(&self, i: usize, n: usize) -> &[TMem] {
        return &self.memory[i..i + n];
    }

    fn step(self) -> Result<MachineState<TMem>, String> {
        if self.halt == true {
            return Err("Machine is halted.");
        }

        let opcode = self.memory[self.instruction_pointer];
        let a = self.memory[self.instruction_pointer + 1];
        let b = self.memory[self.instruction_pointer + 2];
        let loc = self.memory[self.instruction_pointer + 3];

        let mut new_memory = self.memory;

        if opcode == 1 {
            // Addition
            new_memory[loc] = a + b;
            
            return Ok(MachineState {
                halt: self.halt,
                memory: new_memory,
                instruction_pointer: self.instruction_pointer + 4
            });
        }
        else if opcode == 2 {
            // Multiplication
            new_memory[loc] = a * b;

            return Ok(MachineState {
                halt: self.halt,
                memory: new_memory,
                instruction_pointer: self.instruction_pointer + 4
            });
        }
        else if opcode == 99 {
            return Ok(self.halt());
        }
        else {
            return Err("Invalid opcode {} at memory location {}", opcode, self.instruction_pointer);
        }
    }
}

fn make_machine_state<TMemSize: Integer>(memory: Vec<TMemSize>) -> MachineState<TMemSize> {
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
    let machine_state = make_machine_state(get_input_code())
        .write_memory(1, 12)
        .and_then(|m| m.write_memory(2, 2))
        .expect("Error: ");
    
    println!("Memory: {:?}", machine_state.memory);
}
