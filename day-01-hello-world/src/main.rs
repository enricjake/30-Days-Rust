fn main() {
    // Immutable variable (inferred as i32 by default)
    let age = 25;
    println!("I am {} years old.", age);

    // Mutable variable
    let mut temperature = 72.5; // Inferred as f64
    println!("Initial temperature: {}", temperature);

    temperature = 75.0; // Allowed because it is mutable
    println!("Updated temperature: {}", temperature);

    // Constant
    const PLANET: &str = "Earth";
    println!("We are on {}", PLANET);
}