use anyhow::Result;
use std::boxed::Box;
use std::io::{self, BufRead};
use xpath::xpath;

pub type ProcessFn = Box<dyn Fn(&str) -> Option<Vec<String>>>;

pub fn process_stdin(f: ProcessFn) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Getting input from stdin line
        let input = line.unwrap_or_default();

        // Processing input
        let output = f(&input).unwrap_or_default();

        // Stdout
        println!(
            "[{}]",
            output
                .iter()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}

fn main() -> Result<()> {
    process_stdin(Box::new(xpath::xpath_to_array));

    Ok(())
}
