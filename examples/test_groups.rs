use clap::{Arg, Command};
use clap_markdown;

fn main() {
    let app = Command::new("test-groups")
        .about("Test app with grouped options")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .help_heading("General Options")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug output")
                .help_heading("General Options")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input file")
                .help_heading("File Options")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file")
                .help_heading("File Options")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Output format")
                .value_name("FORMAT"), // No help_heading - should go to default "Options"
        );

    // Print the current markdown output
    println!("Current markdown output:");
    println!("{}", clap_markdown::help_markdown_command(&app));

    // Let's also examine the arguments and their help headings
    println!("\n\nArgument analysis:");
    for arg in app.get_arguments() {
        if !arg.is_positional() && !arg.is_hide_set() {
            println!(
                "Arg: {} -> Help heading: {:?}",
                arg.get_id(),
                arg.get_help_heading()
            );
        }
    }
}
