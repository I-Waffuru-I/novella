
use clap::Parser;
use crate::{statics::{SETUP_SEPARATOR,OUTPUT,TOKEN}};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    /// Output file to create
    #[arg(short, long, default_value = OUTPUT)]
    pub output_file_name : String,

    /// Set to print debug 
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug : u8,

    /// Specifies a custom character to use for inline tags. 
    #[arg(short, long, default_value = TOKEN )]
    pub token : String,

    /// Specifies a string to separate the Setup from the Story
    #[arg(short,long, default_value = SETUP_SEPARATOR)]
    pub story_separator : String,


    /// Path towards the input file
    #[arg()]
    pub input_file_path : String,
}
