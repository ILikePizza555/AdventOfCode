fn part1(input: &str) -> u64 {
    return input
        .trim()
        .split("\n")
        .map(|x| str::parse::<u64>(x).unwrap())
        .map(|x| x / 3 - 2)
        .sum::<u64>();
}

fn main() {
    let input_str =  include_str!("input_1.txt");
    println!("Result: {}", part1(input_str));
}
