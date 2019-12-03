static INPUT: &str = include_str!("../input.txt");

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn compute_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn total_fuel(mass: i32) -> i32 {
    let fuel = compute_fuel(mass);

    if fuel <= 0 {
        return 0;
    }

    fuel + total_fuel(fuel)
}

fn main() -> Result<()> {
    let mass_vec: Result<Vec<i32>, _> = INPUT.lines().map(str::parse::<i32>).collect();
    let mass_vec = mass_vec?;

    let fuel: i32 = mass_vec.iter().copied().map(compute_fuel).sum();
    let total_fuel: i32 = mass_vec.iter().copied().map(total_fuel).sum();

    println!("Part 1: {}", fuel);
    println!("Part 2: {}", total_fuel);

    Ok(())
}
