pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // Basic
    println!("{} is from {}", "Brad", "Mass");

    // Positional
    println!("{0} is from {1} and {0} likes ot {2}", "Brad", "Mass", "code");

    // Named
    println!("{name} likes to play {activity}", name = "John", activity = "soccer");

     // Traits
     println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

    // Tuples / Arrays
    println!("{:?}", (12, true, "Hello"));

    // Math
    println!("10+10 = {}", 10 + 10);

}