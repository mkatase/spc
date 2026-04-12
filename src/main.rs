// src/main.rs
use clap::Parser;
use spc::{Cli, Commands};
use spc::check::check_core;
use spc::image::image_core;
use spc::svg::svg_core;
use spc::view::view_core;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Check { name } => {
            println!("Check Mode: {}", name);
            check_core::main(name)
        }
        Commands::View { name } => {
            println!("View Mode: {}", name);
            let _ = view_core::main("view", name);
            Ok(())
        }
        Commands::Run { name } => {
            println!("Run Mode: {}", name);
            let _ = view_core::main("run", name);
            std::process::exit(0);
        }
        Commands::Image { name } => {
            println!("Image Mode: {}", name);
            let _ = image_core::main(name);
            Ok(())
        }
        Commands::Svg { name } => {
            println!("Svg Mode: {}", name);
            let _ = svg_core::main(name);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{}", e); 
        std::process::exit(1);
    }
}
