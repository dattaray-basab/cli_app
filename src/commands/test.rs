pub fn run(verbose: bool, keep: bool) {
    if verbose { println!("[verbose] Running tests..."); }
    if keep { println!("[keep] Persisting functions after test."); }
    println!("Test command executed.");
}
