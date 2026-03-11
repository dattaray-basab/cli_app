pub fn run(pattern: Option<String>, verbose: bool) {
    if verbose { println!("[verbose] Querying..."); }
    match pattern {
        Some(p) => println!("Querying pattern: {}", p),
        None    => println!("Querying all functions."),
    }
}
