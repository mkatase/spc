use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "check mode for spc")]
    Check {
        #[arg(value_name = "FILE", help="input file for spc")]
        name: String,
    },
    #[command(about = "view mode for spc(egui)")]
    View {
        #[arg(value_name = "FILE", help="input file for spc")]
        name: String,
    },
    #[command(about = "run mode for spc(egui)")]
    Run {
        #[arg(value_name = "FILE", help="input file for spc")]
        name: String,
    },
    #[command(about = "image mode for spc(imageproc)")]
    Image {
        #[arg(value_name = "FILE", help="input file for spc")]
        name: String,
    },
    #[command(about = "svg mode for spc")]
    Svg {
        #[arg(value_name = "FILE", help="input file for spc")]
        name: String,
    }
}

pub type CommandMap = HashMap<String, Vec<Vec<String>>>;

//#[warn(dead_code)]
pub struct FileConfig {
    pub height: f32,        // height of screen
    pub width: f32,         // width of screen
    pub extension: String,  // file extension
    pub name: String,       // file name
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            height: 200.0,
            width:  200.0,
            extension: "webp".to_string(),
            name: "spc".to_string(),
        }
    }
}

pub mod canvas;
pub mod check;
pub mod error;
pub mod image;
pub mod svg;
pub mod view;
pub mod utils;
