use clap::{Arg, Command};
use std::fs;
use std::process;
use envy_merge::merge_env_files;

fn main() {
    let matches = Command::new("envy-merge")
        .version("0.1.0")
        .author("Your Name")
        .about("Merges multiple .env files intelligently")
        .arg(Arg::new("files")
            .help("List of .env files to merge")
            .required(true)
            .num_args(2))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output .env file (default: stdout)"))
        .arg(Arg::new("priority")
            .short('p')
            .long("priority")
            .value_name("FILE")
            .help("File to take priority in case of conflicts"))
        .arg(Arg::new("dry-run")
            .short('d')
            .long("dry-run")
            .help("Show merged output without writing to a file"))
        .get_matches();

    let files: Vec<&str> = matches.get_many::<String>("files")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    let priority_file = matches.get_one::<String>("priority");
    let output_file = matches.get_one::<String>("output");
    let dry_run = matches.get_flag("dry-run");

    match merge_env_files(&files, priority_file) {
        Ok(merged_content) => {
            if dry_run {
                println!("{}", merged_content);
            } else if let Some(output) = output_file {
                if let Err(err) = fs::write(output, merged_content) {
                    eprintln!("Error writing to file {}: {}", output, err);
                    process::exit(1);
                }
            } else {
                println!("{}", merged_content);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

