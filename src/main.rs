use std::{env, path::Path, process};

use programming_rust::{ImageShape, parse_complex, parse_pair, render_image};

fn invalid_arg_count(binary_name: &str) -> ! {
    eprintln!("Usage: {binary_name} FILE WIDTHxHEIGHT LEFTxUPPER RIGHTxLOWER");
    eprintln!("Example: {binary_name} mandel.png 1000x750 -1.2,0.35 -1,0.2");
    process::exit(1);
}

fn main() {
    let mut args = env::args();
    let binary_name = args.next().unwrap();
    let image_name = args
        .next()
        .unwrap_or_else(|| invalid_arg_count(&binary_name));
    let bounds = parse_pair::<usize>(
        &args
            .next()
            .unwrap_or_else(|| invalid_arg_count(&binary_name)),
        'x',
    )
    .map(|(width, height)| ImageShape { width, height })
    .expect("Error parsing image dimension");
    let upper_left = parse_complex::<f64>(
        &args
            .next()
            .unwrap_or_else(|| invalid_arg_count(&binary_name)),
    )
    .expect("Error parsing upper left corner point");
    let lower_right = parse_complex::<f64>(
        &args
            .next()
            .unwrap_or_else(|| invalid_arg_count(&binary_name)),
    )
    .expect("Error parsing lower right corner point");

    render_image(Path::new(&image_name), bounds, upper_left, lower_right).unwrap();
}
