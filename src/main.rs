fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let output = args.join(" ");
    println!("{}", output);
}
