use crate::prompt;

pub fn run(kodegen_config_key: Option<String>, verbose: bool, keep: bool, provision: bool) {
    let key = prompt::require(kodegen_config_key, "Config key (function FQN or directory)");
    let keep      = prompt::confirm(keep,      "Persist functions after generation?");
    let provision = prompt::confirm(provision, "Provision workspace after generation?");

    if verbose   { println!("[verbose] Generating..."); }
    if keep      { println!("[keep] Persisting functions after generation."); }
    if provision { println!("[provision] Provisioning workspace after generation."); }
    println!("Generate command executed for key: {}", key);
}
