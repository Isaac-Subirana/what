use std::env;
use std::process::exit;
use colored::Colorize;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
mod print;


fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let args: Vec<String> = env::args().collect();
    let flags: Vec<&String> = args.iter().skip(1).filter(|arg| arg.starts_with('-')).collect();
    let to_find: Vec<&String> = args.iter().skip(1).filter(|arg| !arg.starts_with('-')).collect();

    if args.len() == 1 { //If no arguments are passed, show a mini help menu and point the user to 'what -h'.
        print::mini_help();
        exit(0);
    }

    let valid_flags: [&str; 8] = ["-h", "--help", "-v", "--version", "-m", "--minimal", "-s", "--silent"];

    for flag in &flags {
        if !valid_flags.contains(&flag.as_str()) {
            eprintln!("{} \n{}", "ERROR: You have specified a flag that does not exist.".red(), "You can view the existing flags in the help menu.".yellow());
            exit(1);
        }
    }

    if flags.contains(&&"--help".to_string()) || flags.contains(&&"-h".to_string()) {
        //If --help or -h is passed, check whether other arguments have been passed (to show a warning) and then print the complete help menu.
        if !to_find.is_empty() || flags.len() > 1 {
            eprintln!("{}", "Warning: the other arguments you have specified have been omitted because you have passed --help.".yellow());
        }
        print::help();
        exit(0)
    }

    if flags.contains(&&"--version".to_string()) || flags.contains(&&"-v".to_string()) {
        //If --version or -v is passed, check whether other arguments have been passed (to show a warning) and then print the version number.
        if !to_find.is_empty() || flags.len() > 1 {
            eprintln!("{}", "Warning: the other arguments you have specified have been omitted because you have passed --version.".yellow());
        }
        print::version();
        exit(0)
    }

    let minimal = flags.contains(&&"-m".to_string()) || flags.contains(&&"--minimal".to_string());
    let silent = flags.contains(&&"-s".to_string()) || flags.contains(&&"--silent".to_string());

    if minimal || silent {
        if to_find.is_empty() {
            eprintln!("{}", "ERROR: You should specify what binaries to look for.".red());
            exit(1);
        }
    }

    if minimal && silent {
        eprintln!("{}", "Warning: -s will be omitting -m.".yellow());
    }

    let mut exit_code: i32 = 0;
    
    let mut valid_candidates: usize = 0; //Will be used to check wheter valid candidates have been found.
    let dirs_in_path: Vec<_> = env::var_os("PATH").map(|p| env::split_paths(&p).collect()).unwrap_or_default(); //Read system's PATH.
    #[cfg(windows)]
    let path_extensions: Vec<String> = env::var("PATHEXT").unwrap_or_default().split(";").map(|e| e.to_lowercase()).collect(); //Windows systems' valid PATH file extensions.
    #[cfg(unix)]
    let path_extensions: Vec<String> = vec!["".to_string()]; //Linux systems do not have PATH file extensions, so it is not needed to iterate through them.

    for bin in &to_find { //Iterate through the passed binaries.

        if !minimal && !silent {   
            println!(""); //THIS IS THE THE MOST IMPORTANT LINE! For a nicely formatted output.

            #[cfg(windows)]
            if bin.contains('.') { //Trigger a warning if '.' can be found in any arguments (assuming it could be a file extension).
                let warning_string = format!("{}{}{}{}{}", "WARNING: Will be searching for '", bin, "' with the PATH file extensions (for example, '", bin, ".exe').\nTo avoid this, do not include file extensions in the passed binaries.");
                eprintln!("{}\n", warning_string.yellow());
            }
        }

        //Iterate through the PATH dirs and check whether a file with the passed name and any of the PATH extensions exists (and print its path if it does).
        'PATH_dirs: for dir in &dirs_in_path {
            for ext in &path_extensions {
                let candidate = dir.join(format!("{}{}", bin, ext));
                
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
                        valid_candidates += 1;
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
                println!("{}{}{}", "Could not find '".yellow(), bin, "' in your system's PATH.".yellow());
                exit_code = 1;
            } else if valid_candidates == 1 {
                println!("\n{}", "Found 1 coincidence in your system's PATH.".green());
                valid_candidates = 0;
            } else {
                let multiple_coincidences_string = format!("{}{}{}","Found ", valid_candidates, " coincidences in your system's PATH (which means conflicts could exist with this binary).");
                println!("\n{}", multiple_coincidences_string.green());
                valid_candidates = 0;
            }
        }
    }

    if silent || minimal {
        if valid_candidates == to_find.len() {
            exit(0);
        }
        else {
            exit(1);
        }
    } else {
        exit(exit_code);
    }

}
