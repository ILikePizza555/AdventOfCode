extern crate num;

use num::Integer;

struct MachineState<TMemSize: Integer>
{
    halt: bool,
    memory: Vec<TMemSize>,
    instruction_pointer: usize
}

impl<TMemSize: Integer> MachineState<TMemSize> {
    fn set_instruction_pointer(&mut self, to: usize) {
        if to > self.memory.len() {
            panic!("New instruction pointer {} outside of memory range {}", to, self.memory.len());
        }
        self.instruction_pointer = to;
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
