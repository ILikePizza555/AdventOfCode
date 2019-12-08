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
        if to > self.memory.len() {
            return Error(format!("Write to {} outside of memory range {}", i, self.memory.len()))
        }

        let rv = MachineState {
            halt: self.halt,
            memory: self.memory,
            instruction_pointer: self.instruction_pointer
        };
        rv.memory[i] = d;
        return rv;
    }
    
    fn halt(self) -> MachineState<TMem> {
        return MachineState {
            halt: true,
            memory: self.memory,
            instruction_pointer: self.instruction_pointer
        }
    }

    fn read_memory(&self, i: usize, n: usize) -> &[TMem] {
        return self.memory[i..i + n];
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
    let machine_state = make_machine_state(get_input_code());
    println!("Got {} codes", machine_state.memory.len());
}
