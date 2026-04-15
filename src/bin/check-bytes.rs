#![forbid(unsafe_code)]
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: check-bytes '<condition>' <paths...>");
        std::process::exit(1);
    }

    let cond = &args[1];
    let mut stack: Vec<PathBuf> = args[2..].iter().map(PathBuf::from).collect();

    while let Some(path) = stack.pop() {
        if path.is_dir() {
            let Ok(entries) = fs::read_dir(&path) else {
                continue;
            };
            for entry in entries.flatten() {
                stack.push(entry.path());
            }
            continue;
        }

        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let Some(bytes) = extract_base_block(&content) else {
            continue;
        };

        if eval_condition(cond, &bytes) {
            println!("{}", path.display());
        }
    }
}

fn extract_base_block(content: &str) -> Option<Vec<u8>> {
    let start = content.find("00 ff ff ff ff ff ff 00")?;
    let body = content[start..].split("----------------").next()?;

    Some(
        body.split_whitespace()
            .take(128)
            .map(|s| u8::from_str_radix(s, 16).expect("Invalid hex value"))
            .collect(),
    )
}

fn eval_condition(cond: &str, bytes: &[u8]) -> bool {
    cond.split("&&").all(|clause| {
        let parts: Vec<&str> = clause.split_whitespace().collect();
        assert!(
            parts.len() == 3,
            "Invalid condition format: '{clause}'. Expected 'b<idx> <op> <val>'"
        );

        let idx: usize = parts[0]
            .strip_prefix('b')
            .expect("Index must start with 'b' (e.g., b19)")
            .parse()
            .expect("Invalid index number");

        let op = parts[1];
        let val: u8 = parts[2].parse().expect("Invalid decimal value");

        let b = *bytes.get(idx).expect("EDID index out of bounds");
        match op {
            "==" => b == val,
            "!=" => b != val,
            ">" => b > val,
            "<" => b < val,
            ">=" => b >= val,
            "<=" => b <= val,
            _ => panic!("Unsupported operator: '{op}'"),
        }
    })
}
