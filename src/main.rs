//! # rslogo
//!
//! A simple Logo interpreter which produces the subsequent SVG images.
//!
//! # Example
//! ```shell
//! cargo run <path_to_lg_file> <path_to_image> <height> <width>
//!
//! cargo run examples/flower.lg examples/flower.svg 1000 1000
//! ```
//! This will run the program with the file `examples/flower.lg` and output
//! the image to `examples/flower.svg` with a height and width of 1000.

mod interpreter;
mod parser;

use interpreter::{execute::execute, turtle::Turtle};
use parser::{ast::Expression, parser::parse_tokens, tokenise::tokenize_script};
use std::{collections::HashMap, error::Error, fs::File, io::Read};

use clap::Parser;
use unsvg::Image;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let mut image = Image::new(width, height);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut turtle = Turtle::new(&mut image);

    let mut vars: HashMap<String, Expression> = HashMap::new();
    let tokens = tokenize_script(&contents);
    let ast = parse_tokens(tokens, &mut 0, &mut vars)?;
    execute(&ast, &mut turtle, &mut vars)?;

    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                return Err(format!("Error saving svg: {e}").into());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                return Err(format!("Error saving png: {e}").into());
            }
        }
        _ => {
            return Err("Invalid file extension. Please use .svg or .png".into());
        }
    }

    Ok(())
}
