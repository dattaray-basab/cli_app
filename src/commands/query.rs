pub fn run(patterns: Vec<String>, verbose: bool) {
    if verbose { println!("[verbose] Querying..."); }
    if patterns.is_empty() {
        println!("Querying all functions.");
    } else {
        println!("Querying patterns: {:?}", patterns);
    }
}
