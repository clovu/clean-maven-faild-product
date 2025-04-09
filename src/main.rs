/// This program is a command-line tool for cleaning up failed Maven packages
/// in a local Maven repository. It scans the repository for `.lastUpdated` files,
/// which indicate failed downloads, and removes their parent directories.
///
/// # Features
/// - Automatically detects the Maven repository path based on the operating system.
/// - Allows specifying a custom Maven repository path via a command-line argument.
/// - Provides a silent mode to suppress interactive prompts.
///
/// # Command-Line Arguments
/// - `root`: The root path of the Maven repository. Defaults to `~/.m2` on macOS
///   and `C:\Users\.m2` on Windows.
/// - `--silent` or `-s`: Enables silent mode, skipping interactive prompts.
///
/// # Functions
/// - `expand_tilde`: Expands a tilde-prefixed path (e.g., `~/path`) to an absolute
///   path by replacing the tilde with the user's home directory.
/// - `get_repository_path`: Validates the existence of the Maven repository path
///   and appends the `repository` subdirectory to it.
///
/// # Behavior
/// 1. Parses command-line arguments using the `clap` crate.
/// 2. Validates the Maven repository path and ensures it exists.
/// 3. Scans the repository for `.lastUpdated` files using the `walkdir` crate.
/// 4. Prompts the user to confirm removal of failed packages unless silent mode is enabled.
/// 5. Deletes the parent directories of all `.lastUpdated` files found.
/// 6. Displays progress and success messages using the `indicatif` crate.
///
/// # Error Handling
/// - Panics if the specified Maven repository path does not exist or is invalid.
/// - Returns an `io::Result` to handle file system errors during directory removal.
///
/// # Dependencies
/// - `clap`: For parsing command-line arguments.
/// - `console`: For styling terminal output.
/// - `dirs_next`: For retrieving the user's home directory.
/// - `indicatif`: For displaying progress spinners.
/// - `walkdir`: For recursively walking through directories.
///
/// # Example Usage
/// ```sh
/// # Clean up failed Maven packages in the default repository
/// cargo run
///
/// # Specify a custom Maven repository path
/// cargo run -- /custom/path/to/.m2
///
/// # Run in silent mode
/// cargo run -- --silent
/// ```
///
/// # Note
/// The project is open-sourced on GitHub: https://github.com/clovu/cmvn
use clap::Parser;
use console::Style;
use dirs_next::home_dir;
use indicatif::ProgressBar;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[cfg(target_os = "macos")]
static DEFAULT_DOT_M2: &str = "~/.m2";

#[cfg(target_os = "windows")]
static DEFAULT_DOT_M2: &str = "C:\\Users\\.m2";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Specify the Maven repository root path
    #[arg(default_value_t = DEFAULT_DOT_M2.to_string())]
    root: String,
    /// Whether to always remain silent
    #[arg(long, short, action, default_value_t = false)]
    silent: bool,
}

// Expands a tilde-prefixed path (e.g., "~/path") to an absolute path
// by replacing the tilde with the user's home directory.
fn expand_tilde<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref()
        .to_str()
        .and_then(|s| s.strip_prefix("~"))
        .and_then(|stripped| {
            let trimmed = stripped.trim_start_matches('/');
            home_dir().map(|home| home.join(trimmed))
        })
        .unwrap_or_else(|| path.as_ref().to_path_buf())
}

// Validates the existence of the Maven repository path
// and appends the `repository` subdirectory to it.
fn get_repository_path(path: String) -> PathBuf {
    let red: Style = Style::new().red();

    let repository = expand_tilde(path).join("repository");
    if !repository.exists() || repository.is_file() {
        let panic_msg = format!(
            "This repository {} does not exist.",
            repository.to_str().unwrap()
        );
        panic!("{}", red.apply_to(panic_msg).to_string());
    }
    repository
}

fn main() -> io::Result<()> {
    println!("the project is open-sourced on GitHub: https://github.com/clovu/cmvn");

    let green = Style::new().green();
    let yellow = Style::new().yellow();

    let spinner = ProgressBar::new_spinner();

    let cli = Cli::parse();

    let repository = get_repository_path(cli.root);

    let dirs = WalkDir::new(repository).into_iter().filter_map(Result::ok);
    let mut faild_pack_path: Vec<String> = Vec::new();

    for dir in dirs {
        let file_name = String::from(dir.file_name().to_str().unwrap());
        spinner.set_message(format!("Loading... {}", file_name));

        if file_name.ends_with(".lastUpdated") {
            let parent = dir.path().parent().unwrap();
            let parent: String = String::from(parent.to_string_lossy());
            faild_pack_path.push(parent)
        }
    }

    spinner.finish();

    if !faild_pack_path.is_empty() && !cli.silent {
        let msg = format!(
            "\nFound {} failed packages. Press Enter to continue...",
            faild_pack_path.len()
        );
        let msg = yellow.apply_to(msg).to_string();
        spinner.set_message(msg);

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
    }

    // remove all faild pack of maven
    for folder in &faild_pack_path {
        spinner.set_message(format!("Prepare to remove {}", folder));
        fs::remove_dir_all(folder)?;
        spinner.set_message(format!("Removed {} success", folder));
    }

    let finish_msg = green
        .apply_to(format!(
            "ː̗̀(ꙨꙨ)ː̖́ successful count {} !",
            faild_pack_path.len()
        ))
        .to_string();

    spinner.finish_with_message(finish_msg);

    Ok(())
}
