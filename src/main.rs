use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = match args.get(1) {
        Some(path) if path != "--eval" => fs::read_to_string(path).expect("read source file"),
        _ => args.get(2).cloned().unwrap_or_default(),
    };
    if args.get(1).map(|a| a == "--eval").unwrap_or(false) && args.len() < 3 {
        eprintln!("usage: sphered <file> | sphered --eval '<source>'");
        std::process::exit(1);
    }
    match sphered::eval(&source) {
        Ok(out) => {
            for line in &out.trace {
                println!("{line}");
            }
            println!("=> {}", out.value);
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
