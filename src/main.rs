#![allow(unused_imports)]

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
            // Blur Effect
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount: f32 = args.remove(0).parse().expect("Failed to parse a number");
            blur(infile, outfile, amount);
        }

        "brighten" => {
            // Brightening Effect Added
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount: i32 = args.remove(0).parse().expect("Failed to parse a number");
            brighten(infile, outfile, amount);
        }

        "crop" => {
            // Cropping Feature Here
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let y: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let width: u32 = args.remove(0).parse().expect("Failed to parse a number");
            let height: u32 = args.remove(0).parse().expect("Failed to parse a number");
            crop(infile, outfile, x, y, width, height);
        }

        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let amount: i32 = args.remove(0).parse().expect("Failed to parse a number");
            rotate(infile, outfile, amount);
        }

        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        }

        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        }

        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        "generate" => {
            if args.len() != 4 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            let rm: f32 = args.remove(0).parse().expect("Failed to parse a number");
            let gm: f32 = args.remove(0).parse().expect("Failed to parse a number");
            let bm: f32 = args.remove(0).parse().expect("Failed to parse a number");
            generate(outfile, rm, gm, bm);
        }

        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("Rustic Retouch - a simple image processing tool written in Rust.\n by: Sujoy De\n ");
    println!("USAGE: [subcommand] [infile] [outfile] [args]");
    println!("subcommands:");
    println!("blur INFILE OUTFILE amount(f32)");
    println!("brighten INFILE OUTFILE amount(i32)");
    println!("crop INFILE OUTFILE x(u32) y(u32) width(u32) height(u32)");
    println!("rotate INFILE OUTFILE 90/180/270");
    println!("invert INFILE OUTFILE");
    println!("grayscale INFILE OUTFILE");
    println!("generate OUTFILE red_multiplier(f32) green_multiplier(f32) blue_multiplier(f32)");
    println!("fractal OUTFILE");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, amount: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(amount);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, amount: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = img.brighten(amount);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = img.crop(x, y, width, height);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, amount: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2;
    if amount == 90 {
        img2 = img.rotate90();
    } else if amount == 180 {
        img2 = img.rotate180();
    } else if amount == 270 {
        img2 = img.rotate270();
    } else {
        println!("Please enter a valid amount");
        std::process::exit(-1);
    }
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, rm: f32, gm: f32, bm: f32) {
    let mut imgbuf = image::ImageBuffer::new(800, 800);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = (rm * x as f32) as u8;
        let blue = (bm * y as f32) as u8;
        let green = (gm * x as f32) as u8;
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}
