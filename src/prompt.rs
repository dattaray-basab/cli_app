use dialoguer::{Confirm, Input, theme::ColorfulTheme};

/// Prompt for a required string value if not already provided.
pub fn require(value: Option<String>, label: &str) -> String {
    if let Some(v) = value {
        return v;
    }
    match Input::with_theme(&ColorfulTheme::default())
        .with_prompt(label)
        .interact_text()
    {
        Ok(v) => v,
        Err(_) => plain_read(label),
    }
}

/// Prompt for an optional string value if not already provided.
#[allow(dead_code)]
pub fn ask(value: Option<String>, label: &str) -> Option<String> {
    if value.is_some() {
        return value;
    }
    let prompt = format!("{} (leave blank to skip)", label);
    let result = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(&prompt)
        .allow_empty(true)
        .interact_text();

    let input = match result {
        Ok(v) => v,
        Err(_) => plain_read(&prompt),
    };
    if input.is_empty() { None } else { Some(input) }
}

/// Prompt for a boolean flag if not already set.
pub fn confirm(value: bool, label: &str) -> bool {
    if value {
        return true;
    }
    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(label)
        .default(false)
        .interact()
    {
        Ok(v) => v,
        Err(_) => {
            print!("{} [y/N]: ", label);
            let mut buf = String::new();
            let _ = std::io::stdin().read_line(&mut buf);
            matches!(buf.trim().to_lowercase().as_str(), "y" | "yes")
        }
    }
}

fn plain_read(label: &str) -> String {
    use std::io::Write;
    print!("{}: ", label);
    let _ = std::io::stdout().flush();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read input");
    buf.trim().to_string()
}
