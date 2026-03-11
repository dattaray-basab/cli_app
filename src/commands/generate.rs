pub fn run(kodegen_config_key: String, verbose: bool, keep: bool, provision: bool) {
    if verbose { println!("[verbose] Generating..."); }
    if keep { println!("[keep] Persisting functions after generation."); }
    if provision { println!("[provision] Provisioning workspace after generation."); }
    println!("Generate command executed for key: {}", kodegen_config_key);
}
