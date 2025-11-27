// Example of using the inventory crate
// Compile with: cargo add inventory && cargo run --bin inventory_example

use inventory;

// === Plugin type definition ===
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub handler: fn(&[&str]),
}

impl Command {
    pub const fn new(name: &'static str, description: &'static str, handler: fn(&[&str])) -> Self {
        Command { name, description, handler }
    }
}

// Declares the global command registry
inventory::collect!(Command);

// === Module 1: System Commands ===
mod system {
    use super::Command;

    fn handle_status(_args: &[&str]) {
        println!("  ✓ System operational");
    }

    fn handle_info(_args: &[&str]) {
        println!("  ℹ Version 1.0.0 - Rust Edition");
    }

    // Automatic registration of system module commands
    inventory::submit! {
        Command::new("status", "Display system status", handle_status)
    }

    inventory::submit! {
        Command::new("info", "Display system information", handle_info)
    }
}

// === Module 2: User Commands ===
mod user {
    use super::Command;

    fn handle_login(args: &[&str]) {
        let user = args.get(0).unwrap_or(&"anonymous");
        println!("  🔐 User '{}' logged in", user);
    }

    fn handle_logout(_args: &[&str]) {
        println!("  👋 Successfully logged out");
    }

    inventory::submit! {
        Command::new("login", "Log in a user", handle_login)
    }

    inventory::submit! {
        Command::new("logout", "Log out current user", handle_logout)
    }
}

// === Module 3: Data Commands ===
mod data {
    use super::Command;

    fn handle_save(args: &[&str]) {
        let file = args.get(0).unwrap_or(&"data.txt");
        println!("  💾 Saving to '{}'", file);
    }

    fn handle_load(args: &[&str]) {
        let file = args.get(0).unwrap_or(&"data.txt");
        println!("  📂 Loading from '{}'", file);
    }

    inventory::submit! {
        Command::new("save", "Save data", handle_save)
    }

    inventory::submit! {
        Command::new("load", "Load data", handle_load)
    }
}

// === Main function ===
fn main() {
    println!("=== Distributed Command System with inventory ===\n");

    // List all registered commands (from all modules)
    println!("Available commands:");
    for cmd in inventory::iter::<Command> {
        println!("  • {:8} - {}", cmd.name, cmd.description);
    }

    println!("\n--- Executing some commands ---\n");

    // Execute some commands
    execute_command("status", &[]);
    execute_command("login", &["alice"]);
    execute_command("save", &["backup.db"]);
    execute_command("info", &[]);
    execute_command("logout", &[]);
}

// Find and execute a command by name
fn execute_command(name: &str, args: &[&str]) {
    if let Some(cmd) = inventory::iter::<Command>.into_iter().find(|c| c.name == name) {
        println!("Executing '{}':", name);
        (cmd.handler)(args);
        println!();
    } else {
        println!("❌ Unknown command '{}'\n", name);
    }
}