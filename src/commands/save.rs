pub fn run(pattern: Option<String>, verbose: bool, keep: bool) {
    if verbose { println!("[verbose] Saving..."); }
    if keep { println!("[keep] Persisting functions after save."); }
    match pattern {
        Some(p) => println!("Saving pattern: {}", p),
        None    => println!("Save command executed."),
    }
}
