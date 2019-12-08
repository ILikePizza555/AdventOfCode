struct MachineState
{
    halt: bool,
    memory: Vec<u8>,
    instruction_pointer: usize
}

impl MachineState {
    fn set_instruction_pointer(&mut self, to: usize) {
        if to > self.memory.len() {
            panic!("New instruction pointer {} outside of memory range {}", to, self.memory.len());
        }
        self.instruction_pointer = to;
    }
}

fn make_machine_state(memory: Vec<u8>) -> MachineState {
    return MachineState {
        halt: false,
        memory: memory,
        instruction_pointer: 0
    }
}

fn get_input_code() -> Vec<u8> {
    return include_str!("input1.csv")
        .trim()
        .split(",")
        .map(|x| str::parse::<u8>(x).unwrap())
        .collect::<Vec::<u8>>();
}

fn main() {
    let machine_state = make_machine_state(get_input_code());
    println!("Got {} codes", machine_state.memory.len());
}
