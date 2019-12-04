fn get_input_masses() -> Vec<u64> {
    return include_str!("input_1.txt")
        .trim()
        .split("\n")
        .map(|x| str::parse::<u64>(x).unwrap())
        .collect::<Vec<u64>>();
}

fn mass_to_fuel_total(mass: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut fuel: u64 = mass;

    return loop {
        fuel = if fuel > 6 { fuel / 3 - 2 } else { 0 };
        sum += fuel;

        if fuel == 0 {
            break sum;
        }
    }
}

fn main() {
    let input_masses = get_input_masses();

    let module_fuel = input_masses
        .iter()
        .cloned()
        .map(|x| x / 3 - 2)
        .sum::<u64>();

    let total_fuel = input_masses
        .iter()
        .cloned()
        .map(mass_to_fuel_total)
        .sum::<u64>();

    println!("Module fuel: {}; Total fuel: {}", module_fuel, total_fuel);
}
