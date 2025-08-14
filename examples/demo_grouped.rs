use clap::{Arg, Command};

fn main() {
    let app = Command::new("demo-grouped")
        .about("Demo application showing grouped options")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .help_heading("Logging Options")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress output")
                .help_heading("Logging Options")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input file")
                .help_heading("File Operations")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file")
                .help_heading("File Operations")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .help("Number of threads")
                .help_heading("Performance Options")
                .value_name("NUM"),
        )
        .arg(
            Arg::new("memory")
                .short('m')
                .long("memory")
                .help("Memory limit")
                .help_heading("Performance Options")
                .value_name("SIZE"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file")
                .value_name("FILE"), // No help_heading - goes to default "Options"
        );

    println!("{}", clap_markdown::help_markdown_command(&app));
}
