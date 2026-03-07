use std::env;
use colored::Colorize;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn print_help() {
    println!("\n{}", "Warning: passing --help will omit all other arguments you might have specified.".yellow());
    println!("{}", "          _           _".magenta());
    println!("{}", "         | |         | |".magenta());
    println!("{}", "__      _| |__   __ _| |_".magenta());
    println!("{}", "\\ \\ /\\ / / '_ \\ / _` | __|".magenta());
    println!("{}", " \\ V  V /| | | | (_| | |_".magenta());
    println!("{}{}", "  \\_/\\_/ |_| |_|\\__,_|\\__|".magenta(), " : an adaptation of Linux's 'which', coded in Rust.".cyan());
    println!("\n{}", "Usage:".cyan());
    println!(" {} [option] <binary/ies>", "what".magenta());
    println!("\n{}", "Options:".cyan());
    println!("\t{} {} {}", "Pass".cyan(), "-m".magenta(), "for the minimal, non-colored, 'which'-like output.".cyan());
    println!("\t{} {} {}", "Pass".cyan(), "-s".magenta(), "to silently iterate through the passed arguments. If at least one occurrence of each one is found, it returns 0; if not, it returns 1.".cyan());
    #[cfg(windows)]
    println!("\n{}", "Searches for all of the occurrences of the passed binary or binaries (with all of the system's path extensions) in the system's PATH.".cyan());
    #[cfg(unix)]
    println!("\n{}", "Searches for all of the occurrences of the passed binary or binaries in the system's PATH.".cyan());
    #[cfg(windows)]
    println!("{}", "Do not include the file extension of the binaries to search (to search for 'explorer.exe', just type in 'which explorer', as this will avoid searching for 'explorer.exe.exe').".yellow());
    #[cfg(unix)]
    println!("");
}

fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let args: Vec<String> = env::args().collect();
    let num_args: usize = env::args().skip(1).count();

    if num_args == 0 { //If no arguments are passed, show a mini help menu and point the user to 'what -h'.
        println!("{}", "          _           _".magenta());
        println!("{}", "         | |         | |".magenta());
        println!("{}", "__      _| |__   __ _| |_".magenta());
        println!("{}", "\\ \\ /\\ / / '_ \\ / _` | __|".magenta());
        println!("{}", " \\ V  V /| | | | (_| | |_".magenta());
        println!("{}", "  \\_/\\_/ |_| |_|\\__,_|\\__|".magenta());
        println!("\n{}", "Pass -h (or --help) to view the entire help menu.".yellow());
        println!("\n{}", "Usage:".cyan());
        println!(" {} {}", "what".magenta(), "[options] <binary/ies>".cyan());
        #[cfg(unix)]
        println!("");
        std::process::exit(0);
    }

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) { //If --help or -h is passed, print the help menu as well.
        print_help();
        std::process::exit(0)
    }

    let minimal = args.contains(&"-m".to_string());
    let silent = args.contains(&"-s".to_string());

    if minimal && silent {
        if num_args == 2 {
            println!("{}", "ERROR: You should specify what binaries to look for.".red());
            std::process::exit(1);
        }
        println!("{}", "Warning: -s will be omitting -m.".yellow());
    }

    if minimal || silent {
        if num_args == 1 {
            println!("{}", "ERROR: You should specify what binaries to look for.".red());
            std::process::exit(1);
        } 
    }

    let mut exit_code: i32 = 0;
    
    let mut valid_candidates: usize = 0; //Will be used to check wheter valid candidates have been found.
    let dirs_in_path: Vec<_> = env::var_os("PATH").map(|p| env::split_paths(&p).collect()).unwrap_or_default(); //Read system's PATH.
    #[cfg(windows)]
    let path_extensions: Vec<String> = env::var("PATHEXT").unwrap_or_default().split(";").map(|e| e.to_lowercase()).collect(); //Windows systems' valid PATH file extensions.
    #[cfg(unix)]
    let path_extensions: Vec<String> = vec!["".to_string()]; //Linux systems do not have PATH file extensions, so it is not needed to iterate through them.

    for i in 0 .. num_args { //Iterate through the passed arguments.

        if !minimal && !silent {   
            println!(""); //THIS IS THE THE MOST IMPORTANT LINE! For a nicely formatted output.

            #[cfg(windows)]
            if args[i + 1].contains('.') { //Trigger a warning if '.' can be found in any arguments (assuming it could be a file extension).
                let warning_string = format!("{}{}{}{}{}", "WARNING: Will be searching for '", &args[i + 1], "' with the PATH file extensions (for example, '",
                    &args[i + 1], ".exe').\nTo avoid this, do not include file extensions in the passed binaries.");
                println!("{}\n", warning_string.yellow());
            }
        }

        //Iterate through the PATH dirs and check whether a file with the passed name and any of the PATH extensions exists (and print its path if it does).
        'PATH_dirs: for dir in &dirs_in_path {
            for ext in &path_extensions {
                let candidate = dir.join(format!("{}{}", args[i + 1], ext));
                
                #[cfg(windows)]
                let is_valid = candidate.is_file();
                #[cfg(unix)]
                let is_valid = candidate.is_file() && candidate.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false);

                if is_valid { //If -s has been passed, do not print anything but count the valid candidates that have been found.
                    if silent {
                        valid_candidates += 1;
                        break 'PATH_dirs;
                    }
                    else if minimal { //If -m has been passed, print only one occurence for each argument passed (if found), without coloring.
                        println!("{}", candidate.display());
                        break 'PATH_dirs;
                    }
                    else {
                        valid_candidates += 1;
                        let found_candidate_string = format!("{}", candidate.display());
                        println!("{}", found_candidate_string.cyan());
                    }
                }
            }
        } 

        if !minimal && !silent { //If -m has not been passed, show custom messages depending on the amount of coincidences found:
            if valid_candidates == 0 {
                println!("{}", "Found no coincidences in your system's PATH.".green());
                exit_code = 1;
            } else if valid_candidates == 1 {
                println!("\n{}", "Found 1 coincidence in your system's PATH.".green());
                valid_candidates = 0;
            } else {
                let multiple_coincidences_string = format!("{}{}{}","Found ", valid_candidates, " coincidences in your system's PATH (which means conflicts could exist with this binary).");
                println!("\n{}", multiple_coincidences_string.green());
                valid_candidates = 0;
            }
            #[cfg(unix)]
            println!(""); //ANOTHER REALLY IMPORTANT LINE! Again, for a nicely formatted output :D
        }
    }

    if silent || minimal {
        if valid_candidates == num_args - 1 {
            std::process::exit(0);
        }
        else {
            std::process::exit(1);
        }
    } else {
        std::process::exit(exit_code);
    }

}
