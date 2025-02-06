use std::io;

fn main() {
    // Prompt user for base
    println!("Enter the base number:");
    let mut base_input = String::new();
    io::stdin().read_line(&mut base_input).expect("Failed to read line");
    let base: u64 = base_input.trim().parse().expect("Please enter a valid number");

    // Prompt user for exponent
    println!("Enter the exponent:");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read line");
    let exp: u32 = exp_input.trim().parse().expect("Please enter a valid number");

    // Calculate the power
    let number = base.pow(exp);

    // Display result
    println!("{} raised to the {} power = {}", base, exp, number);
}