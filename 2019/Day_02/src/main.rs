fn get_input_code() -> Vec<u8> {
    return include_str!("input1.csv")
        .trim()
        .split(",")
        .map(|x| str::parse::<u8>(x).unwrap())
        .collect::<Vec::<u8>>();
}

fn main() {
    let input_code = get_input_code();
    println!("Got {} codes", input_code.len());
}
