use colored::Colorize;

pub fn mini_help() {
    println!("{}", "          _           _".magenta());
    println!("{}", "         | |         | |".magenta());
    println!("{}", "__      _| |__   __ _| |_".magenta());
    println!("{}", "\\ \\ /\\ / / '_ \\ / _` | __|".magenta());
    println!("{}", " \\ V  V /| | | | (_| | |_".magenta());
    println!("{}", "  \\_/\\_/ |_| |_|\\__,_|\\__|".magenta());
    println!("\n{}", "Pass -h (or --help) to view the entire help menu.".yellow());
    println!("\n{}", "Usage:".cyan());
    println!("\t{} {} {}", "what".magenta(), "[option]", "<binary/ies>".cyan());
}

pub fn help() {
    println!("{}", "          _           _".magenta());
    println!("{}", "         | |         | |".magenta());
    println!("{}", "__      _| |__   __ _| |_".magenta());
    println!("{}", "\\ \\ /\\ / / '_ \\ / _` | __|".magenta());
    println!("{}", " \\ V  V /| | | | (_| | |_".magenta());
    println!("{}{}", "  \\_/\\_/ |_| |_|\\__,_|\\__|".magenta(), " : an adaptation of Linux's 'which', coded in Rust.".cyan());
    #[cfg(windows)]
    println!("\n{}", "Searches for all of the occurrences of the passed binary or binaries (with all of the system's path extensions) in the system's PATH.".cyan());
    #[cfg(unix)]
    println!("\n{}", "Searches for all of the occurrences of the passed binary or binaries in the system's PATH.".cyan());
    println!("\n{}", "Usage:".cyan());
    println!("\t{} {} {}", "what".magenta(), "[option]", "<binary/ies>".cyan());
    println!("\n{}", "Options:".cyan());
    println!("\t{} {} {}", "Pass".cyan(), "-h / --help".magenta(), "to view this help menu and exit with code 0.".cyan());
    println!("\t{} {} {}", "Pass".cyan(), "-v / --version".magenta(), "to print the version and exit with code 0.".cyan());
    println!("\t{} {} {}", "Pass".cyan(), "-m / --minimal".magenta(), "for the minimal, non-colored, 'which'-like output.".cyan());
    println!("\t{} {} {}", "Pass".cyan(), "-s / --silent".magenta(), "to silently iterate through the passed arguments. If at least one occurrence of each one is found, it returns 0; otherwise, it returns 1.".cyan());
    #[cfg(windows)]
    println!("{}", "Do not include the file extension of the binaries to search (to search for 'explorer.exe', just type in 'which explorer', as this will avoid searching for 'explorer.exe.exe').".yellow());
}

pub fn version() {
    println!("{} {}", "what".magenta(), "v1.1.0".cyan());
}
