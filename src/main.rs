

//! Novella is a tool to simplify styling narrative writing.
//! It expects a file path as input and will write the results to `./output.pdf`, creating if
//! needed, overwriting if needed.

use std::{fs::read_to_string, io::Write};
use crate::{
    args::Args, builder::Builder, parser::Parser, types::StoryError
};
use clap::Parser as cParser;
use tectonic;

mod parser;
mod builder;
mod types;
mod args;
mod statics;

#[cfg(test)]
mod tests;



fn main() {

    let args = Args::parse();
    
    if let Err(e) = run(&args) {
        eprintln!("Failed to run program: \n{}",e);
    } 

}

fn run(args : &Args) -> Result<(), StoryError> {
    let mut parser = Parser::new(&args.story_separator,get_token(args)?);
    let builder = Builder::new();
    // tex.0 = setup, tex.1 = content
    let rslt = read_to_string(args.input_file_path.clone()).expect(&format!("Couldn't read file at path {}",args.input_file_path));

    let tokens = parser.tokenize(rslt)?;
    let built = builder.build_stack(tokens)?;
    let pdf = tectonic::latex_to_pdf(&built).unwrap();
    let mut file = std::fs::File::create(args.output_file_name.clone()).unwrap();
    println!("Created file");
    file.write(&pdf).unwrap();

    Ok(())
} 

fn get_token(args : &Args) -> Result<char,StoryError>{
    match args.token.chars().nth(0) {
        Some(c) => Ok(c),
        None => Err(StoryError::ParsingError("No 0th char in token [".to_string() + &args.token + "]"))
    }
}

