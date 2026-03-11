pub fn run(patterns: Vec<String>, verbose: bool) {
    if verbose { println!("[verbose] Clearing..."); }
    if patterns.is_empty() {
        println!("Database reset.");
    } else {
        println!("Clearing patterns: {:?}", patterns);
    }
}
