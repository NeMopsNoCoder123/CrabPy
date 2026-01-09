use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;
use pyo3::prelude::*;
use pyo3::types::PyDict;

fn main() -> PyResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("🦀 CrabPy v1.1 | Rust Copiler for python");
        println!("Usage: crabpy <file.crpy>");
        return Ok(());
    }

    let file_path = &args[1];
    let path = Path::new(file_path);

    if path.extension().and_then(|s| s.to_str()) != Some("crpy") {
        println!("❌ Error: CrabPy support only .crpy extension files.");
        return Ok(());
    }

    let mut code = fs::read_to_string(file_path).expect("Unsuccessful file read");

    let replacements = [
        (r"\bgrab\b", "import"), (r"\bfn\b", "def"), (r"\btrue\b", "True"),
        (r"\bfalse\b", "False"), (r"\bnull\b", "None"), (r"\bask\b", "input"),
        (r"\bprint\b", "print"), (r"\bif\b", "if"), (r"\belif\b", "elif"), 
        (r"\belse\b", "else"), (r"\bfor\b", "for"), (r"\bwhile\b", "while"), 
        (r"\bin\b", "in"), (r"\bis\b", "is"), (r"\band\b", "and"), 
        (r"\bor\b", "or"), (r"\bnot\b", "not"), (r"\bas\b", "as"), 
        (r"\bfrom\b", "from"), (r"\bclass\b", "class"), (r"\btry\b", "try"), 
        (r"\bexcept\b", "except"), (r"\bfinally\b", "finally"), (r"\bwith\b", "with"), 
        (r"\breturn\b", "return"), (r"\byield\b", "yield"), (r"\bbreak\b", "break"), 
        (r"\bcontinue\b", "continue"), (r"\bpass\b", "pass"), (r"\braise\b", "raise"), 
        (r"\bassert\b", "assert"), (r"\bglobal\b", "global"), (r"\bnonlocal\b", "nonlocal"), 
        (r"\blambda\b", "lambda"), (r"\bdel\b", "del"), (r"\basync\b", "async"), 
        (r"\bawait\b", "await")
    ];

    for (pattern, replacement) in replacements.iter() {
        let re = Regex::new(pattern).unwrap();
        code = re.replace_all(&code, *replacement).to_string();
    }

    let mut final_code = String::new();
    let mut indent_level = 0;

    for line in code.lines() {
        let mut trimmed = line.trim().to_string();
        if trimmed.is_empty() { continue; }

        if trimmed.starts_with('}') {
            indent_level = indent_level.saturating_sub(1);
            trimmed = trimmed[1..].trim().to_string();
        }

        if !trimmed.is_empty() {
            final_code.push_str(&"    ".repeat(indent_level));

            if trimmed.ends_with('{') {
                let clean = trimmed.trim_end_matches('{').trim();
                final_code.push_str(clean);
                final_code.push(':');
                indent_level += 1;
            } else {
                final_code.push_str(&trimmed);
            }
            final_code.push('\n');
        }
    }

    Python::with_gil(|py| {
        let globals = PyDict::new(py);
        let builtins = py.import("builtins")?;
        globals.set_item("__builtins__", builtins)?;

        py.run(&final_code, Some(globals), Some(globals))
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
                e
            })
    })?;

    Ok(())
}
