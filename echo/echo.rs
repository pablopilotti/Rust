fn main() {
    // Collect command line arguments into a vector, skipping the first one (program name)
    let args: Vec<String> = std::env::args().skip(1).collect();
    // Join the arguments into a single string separated by spaces
    let output = args.join(" ");
    // Print the resulting string to the console
    println!("{}", output);
}
