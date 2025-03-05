use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version = "1.0",
    author = "Leo Dev",
    about = "A simple Marco Polo game"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "play")]
    Play {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Play { name }) => {
            let result: String = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No command provided"),
    }
}
