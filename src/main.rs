use clap::{Parser, Subcommand};
use rand::Rng;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
    process::Command,
};
use termimad::{rgb, MadSkin};

#[derive(Parser)]
#[clap(
    name = "Giru",
    version = "1.0",
    author = "WMouton",
    about = "A simple memory-saving tool"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all saved memories
    List,
    /// Open the memory file with Neovim in Alacritty
    Open,
    /// View the memory file with Frogmouth
    View,
    /// Save a new memory
    Save,
    /// Open a path or file with Obsidian or Neovim
    Obsidian { path: Option<String>, neovim: bool },
    /// Display help information
    Help,
    /// Display the author of the tool
    Author,
}

fn main() {
    let cli = Cli::parse();
    let giru_path = format!("{}/.giru/giru.md", std::env::var("HOME").unwrap());

    match &cli.command {
        Some(Commands::List) => list_giru(&giru_path),
        Some(Commands::Open) => open_giru(&giru_path),
        Some(Commands::View) => view_giru(&giru_path),
        Some(Commands::Save) | None => save_memory(&giru_path),
        Some(Commands::Obsidian { path, neovim }) => open_with_obidian_or_neovim(path, *neovim),
        Some(Commands::Help) => display_help(),
        Some(Commands::Author) => println!("🤖 Author: WMouton"),
    }
}

fn list_giru(giru_path: &str) {
    if !Path::new(giru_path).exists() {
        eprintln!("🤖 (Giru): No memories found. Please save a memory first.");
        return;
    }

    let file_contents = fs::read_to_string(giru_path)
        .expect("Error occurred, could not read giru file");

    println!("🤖 Giru Contents Below 🤖");
    let skin = make_terminal_skin();
    skin.print_text(&file_contents);
}

fn open_giru(giru_path: &str) {
    if !Path::new(giru_path).exists() {
        eprintln!("🤖 (Giru): No memories found. Please save a memory first.");
        return;
    }

    Command::new("alacritty")
        .arg("-e")
        .arg("nvim")
        .arg(giru_path)
        .spawn()
        .expect("Failed to open file with Neovim in Alacritty");
}

fn view_giru(giru_path: &str) {
    if !Path::new(giru_path).exists() {
        eprintln!("🤖 (Giru): No memories found. Please save a memory first.");
        return;
    }

    let status = Command::new("frogmouth")
        .arg(giru_path)
        .status()
        .expect("Failed to view file with Frogmouth");

    if !status.success() {
        eprintln!("🤖 (Giru): Failed to view file with Frogmouth.");
    }
}

fn open_with_obidian_or_neovim(path: &Option<String>, neovim: bool) {
    if neovim {
        Command::new("alacritty")
            .arg("-e")
            .arg("nvim")
            .arg(path.as_deref().unwrap_or("~"))
            .spawn()
            .expect("Failed to open path with Neovim in Alacritty");
    } else {
        let command = "obsidian";
        let arg = path.as_deref().unwrap_or("~");

        Command::new(command)
            .arg(arg)
            .spawn()
            .expect("Failed to open path with Obsidian");
    }
}

fn save_memory(file_location: &str) {
    if !Path::new(file_location).exists() {
        let prefix = Path::new(file_location).parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
        File::create(file_location).expect("Failed to create file, please try again");
    }

    let mut md_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_location)
        .unwrap();

    println!("🤖 (Giru): What do you want to remember?");
    let thing_to_remember = read_input();

    println!("🤖 (Giru): Enter Title of this memory:");
    let why_store_this_thing = read_input();

    if Path::new(file_location).metadata().unwrap().len() == 0 {
        writeln!(md_file, "# Your 🤖 Giru File").expect("Failed to write to file, please try again");
    }

    writeln!(md_file, "## {}", why_store_this_thing).expect("Failed to write to file, please try again");
    writeln!(md_file, "```\n{}```\n", thing_to_remember).expect("Failed to write to file, please try again");

    let hint = if rand::thread_rng().gen_range(1..=2) == 1 {
        "list"
    } else {
        "view"
    };

    println!(
        "🤖 (Giru): I've saved the new item.\n Hint: use `giru {}` to open all my memories.",
        hint
    );
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read your request. Please try again");
    input.trim().to_string()
}

fn make_terminal_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(246, 189, 0));
    skin
}

fn display_help() {
    println!("🤖 Giru Help Menu:");
    println!("  giru list           - List all saved memories");
    println!("  giru open           - Open the memory file with Neovim in Alacritty");
    println!("  giru view           - View the memory file with Frogmouth");
    println!("  giru save           - Save a new memory");
    println!("  giru obsidian [path] - Open the specified path or file with Obsidian");
    println!("  giru obsidian [path] --neovim - Open the specified path or file with Neovim in Alacritty");
    println!("  giru help           - Display this help menu");
    println!("  giru author         - Display the author of the tool");
}
