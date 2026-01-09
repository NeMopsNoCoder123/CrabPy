use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("🍤 Shrimp: net <pkg> | toss <pkg>");
        return;
    }

    let action = if args[1] == "net" { "install" } else { "uninstall" };
    let pkg = &args[2];

    let mut cmd = Command::new("pip");
    cmd.arg(action).arg(pkg);
    if action == "uninstall" { cmd.arg("-y"); }

    match cmd.status() {
        Ok(_) => println!("🍤 Shrimp: {} {} виконано!", pkg, action),
        Err(e) => println!("Помилка: {}", e),
    }
}