use super::parser::Cli;
use arboard::Clipboard;
use std::process::{Command, exit};

pub fn orchestrate_commit(cli: &Cli, message: &str) {
    // Automatically run git add -A
    println!("➡️ Running git add -A");
    run(cli, "git", &["add", "-A"]);

    if cli.clipboard {
        println!("📋 Copying commit message to clipboard");
        if !cli.dry_run {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(message).unwrap();
        }
    } else {
        let args = if cli.sign {
            vec!["commit", "-S", "-m", message]
        } else {
            vec!["commit", "-m", message]
        };

        let command_string = format!("git {}", args.join(" "));
        println!("➡️ Running {}", command_string);
        run(cli, "git", &args);

        if cli.push {
            println!("➡️ Running git push");
            run(cli, "git", &["push"]);
        }
    }

    println!("\n✅ Done!");
}

pub fn run(cli: &Cli, command: &str, args: &[&str]) {
    if cli.dry_run {
        return;
    }

    let status = Command::new(command)
        .args(args)
        .status()
        .expect("❌ Failed to execute command. Is git installed and in your PATH?");

    if !status.success() {
        let full_command = format!("`{} {}`", command, args.join(" "));
        
        eprintln!(
            "\n❌ Error: Command {} failed with exit code {}. Halting execution.",
            full_command,
            status.code().unwrap_or(1)
        );
        
        exit(1);
    }
}
