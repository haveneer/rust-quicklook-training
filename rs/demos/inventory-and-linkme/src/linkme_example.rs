// Example of using the linkme crate
// Run with: cargo run --bin linkme_example

use linkme::distributed_slice;

// === Plugin type definition ===
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub handler: fn(&[&str]),
}

impl Command {
    pub const fn new(name: &'static str, description: &'static str, handler: fn(&[&str])) -> Self {
        Command {
            name,
            description,
            handler,
        }
    }
}

// Declares the distributed slice of commands
#[distributed_slice]
pub static COMMANDS: [Command];

// === Module 1: System Commands ===
mod system {
    use super::{COMMANDS, Command};
    use linkme::distributed_slice;

    fn handle_status(_args: &[&str]) {
        println!("  ✓ System operational");
    }

    fn handle_info(_args: &[&str]) {
        println!("  ℹ Version 1.0.0 - Rust Edition");
    }

    // Automatic registration of system module commands
    #[distributed_slice(COMMANDS)]
    static CMD_STATUS: Command = Command::new("status", "Display system status", handle_status);

    #[distributed_slice(COMMANDS)]
    static CMD_INFO: Command = Command::new("info", "Display system information", handle_info);
}

// === Module 2: User Commands ===
mod user {
    use super::{COMMANDS, Command};
    use linkme::distributed_slice;

    fn handle_login(args: &[&str]) {
        let user = args.first().unwrap_or(&"anonymous");
        println!("  🔐 User '{}' logged in", user);
    }

    fn handle_logout(_args: &[&str]) {
        println!("  👋 Successfully logged out");
    }

    #[distributed_slice(COMMANDS)]
    static CMD_LOGIN: Command = Command::new("login", "Log in a user", handle_login);

    #[distributed_slice(COMMANDS)]
    static CMD_LOGOUT: Command = Command::new("logout", "Log out current user", handle_logout);
}

// === Module 3: Data Commands ===
mod data {
    use super::{COMMANDS, Command};
    use linkme::distributed_slice;

    fn handle_save(args: &[&str]) {
        let file = args.first().unwrap_or(&"data.txt");
        println!("  💾 Saving to '{}'", file);
    }

    fn handle_load(args: &[&str]) {
        let file = args.first().unwrap_or(&"data.txt");
        println!("  📂 Loading from '{}'", file);
    }

    #[distributed_slice(COMMANDS)]
    static CMD_SAVE: Command = Command::new("save", "Save data", handle_save);

    #[distributed_slice(COMMANDS)]
    static CMD_LOAD: Command = Command::new("load", "Load data", handle_load);
}

// === Main function ===
fn main() {
    println!("=== Distributed Command System with linkme ===\n");

    // List all registered commands (from all modules)
    println!("Available commands:");
    for cmd in COMMANDS {
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
    if let Some(cmd) = COMMANDS.iter().find(|c| c.name == name) {
        println!("Executing '{}':", name);
        (cmd.handler)(args);
        println!();
    } else {
        println!("❌ Unknown command '{}'\n", name);
    }
}
