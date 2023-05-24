use clap::Parser;

pub mod add;
pub mod cli;
pub mod init;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
    	Some(Commands::Init { mkdir }) => {
    		println!("Init: {}", mkdir);
    		init::init();
    	}
    	Some(Commands::Add { path }) => {
    		println!("Add: {}", path);
    		add::add(&path);
    	}
    	None => {}
    }
}
