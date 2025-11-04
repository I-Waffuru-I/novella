use std::{env,fs::read_to_string, io::Write};
use tectonic;

use crate::{
    parser::Parser,
    builder::Builder,
    types::StoryError,
}
;

mod parser;
mod builder;
mod types;
#[cfg(test)]
mod tests;



fn main() {

    let file_paths: Vec<String> = env::args().collect();
    if file_paths.iter().count() < 2 {
        eprintln!("Provide a file path!");
        return
    }
    if let Err(e) = run(&file_paths[1],"./output.pdf") {
        eprintln!("Failed to run program: {}",e);
        return
    } 
}

fn run(path : &str, output : &str) -> Result<(), StoryError> {
    let mut parser = Parser::new();
    let builder = Builder::new();
    // tex.0 = setup, tex.1 = content
    let rslt = read_to_string(path).expect(&format!("Couldn't read file at path {}",path));

    let tokens = parser.tokenize(rslt)?;
    let built = builder.build_stack(tokens)?;
    let pdf = tectonic::latex_to_pdf(&built).unwrap();
    println!("Output length: [{}]",pdf.len());
    let mut file = std::fs::File::create(output).unwrap();
    println!("Created file");
    file.write(&pdf).unwrap();
    
    Ok(())
} 

