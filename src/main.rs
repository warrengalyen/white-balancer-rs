extern crate image;
extern crate clap;
extern crate white_balancer;

use std::fs::File;
use std::path::Path;

use white_balancer::traits::AutoWhiteBalance;

fn main() {
    let matches = clap::App::new("white-balancer")
        .version("0.1.0")
        .author("Warren Galyen <warrengalyen@github.com>")
        .about("Automatic white balance for images")
        .arg(clap::Arg::with_name("input")
            .help("input image filename")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true)
        )
        .arg(clap::Arg::with_name("output")
            .help("output image filename")
            .short("o")
            .long("output")
            .takes_value(true)
            .required(true)
        )
        .arg(clap::Arg::with_name("method")
            .help("white balancing method")
            .short("m")
            .long("method")
            .takes_value(true)
            .required(false)
        )
        .get_matches ();

        let input_filename = matches.value_of("input").unwrap();
        let output_filename = matches.value_of("output").unwrap();
        let method = match matches.value_of("method") {
            Some(method) => {
                method
            },
            None => {
                "gray-world"
            }
        };

        println!("Auto white balancing:");
        println!("\tMethod: {}", method);
        println!("\tInput: {}", input_filename);
        println!("\tOutput: {}", output_filename);

        let original_image = image::open(&input_filename)
        .unwrap();
    let orig_rgb = original_image.to_rgb();
    let enhanced_image = match method {
        "gray-world" => {
            Some(white_balancer::GrayWorld::white_balance(&orig_rgb))
        },
        _ => {
            eprintln!("Auto white balancing method '{}' not found", method);
            None
        }
    };

    match enhanced_image {
        Some(enh_image) => {
            image::DynamicImage::ImageRgb8(enh_image).save(&Path::new(output_filename))
                .unwrap();
            },
            None => {
                eprintln!("Failed to convert with method: '{}'", method);
        }
    }
}