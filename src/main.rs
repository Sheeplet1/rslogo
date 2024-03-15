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

pub mod errors;
mod interpreter;
mod parser;

use parser::ast::Expression;
use std::{collections::HashMap, fs::File, io::Read};

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

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let mut image = Image::new(width, height);

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut turtle = interpreter::turtle::Turtle {
        x: (width / 2) as f32,
        y: (height / 2) as f32,
        heading: 0,
        pen_down: false,
        pen_color: 7, // White
        image: &mut image,
    };

    let mut variables: HashMap<String, Expression> = HashMap::new();
    let tokens = parser::parse::tokenize_script(&contents);
    let ast = parser::parse::parse_tokens(tokens, &mut variables).unwrap();

    match interpreter::execute::execute(&ast, &mut turtle, &mut variables) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}");
        }
    }

    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}
