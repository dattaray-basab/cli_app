use crate::prompt;

pub fn run(expression: Option<String>, verbose: bool) {
    let expression = prompt::require(expression, "Expression to evaluate");
    if verbose { println!("[verbose] Watching..."); }
    println!("Watch mode: {}", expression);
}
