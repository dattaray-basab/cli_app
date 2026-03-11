use crate::prompt;

pub fn run(function: Option<String>, params: Vec<String>, verbose: bool) {
    let function = prompt::require(function, "Function name or FQN");
    if verbose { println!("[verbose] Invoking..."); }
    println!("Invoking {} with params: {:?}", function, params);
}
