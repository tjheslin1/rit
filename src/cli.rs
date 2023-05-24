use clap::{Parser, Subcommand};

#[derive(Parser)]
// #[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// initialise .rit/
    Init {
        /// lists test values
        #[arg(short, long)]
        mkdir: bool,
    },

    Add {
    	path: String,
    }
}
