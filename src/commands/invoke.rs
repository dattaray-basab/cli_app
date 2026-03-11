pub fn run(function: String, params: Vec<String>, verbose: bool) {
    if verbose { println!("[verbose] Invoking..."); }
    println!("Invoking {} with params: {:?}", function, params);
}
