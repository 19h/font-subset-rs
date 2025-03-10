use std::env;
use std::fs;
use std::io::{self, Read, Write};
use woff2_sys::convert_woff2_to_ttf;

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if we have the correct number of arguments
    if args.len() != 3 {
        eprintln!("Usage: {} <input.woff2> <output.ttf>", args[0]);
        return Ok(());
    }
    
    let input_path = &args[1];
    let output_path = &args[2];
    
    // Read the WOFF2 file
    println!("Reading WOFF2 file: {}", input_path);
    let mut woff2_data = Vec::new();
    let mut file = fs::File::open(input_path)?;
    file.read_to_end(&mut woff2_data)?;
    
    // Convert WOFF2 to TTF
    println!("Converting WOFF2 to TTF...");
    let ttf_data = match convert_woff2_to_ttf(&woff2_data) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error: Failed to convert WOFF2 to TTF");
            return Err(io::Error::new(io::ErrorKind::Other, "Conversion failed"));
        }
    };
    
    // Write the TTF file
    println!("Writing TTF file: {}", output_path);
    let mut output_file = fs::File::create(output_path)?;
    output_file.write_all(&ttf_data)?;
    
    println!("Conversion successful!");
    println!("Input size: {} bytes", woff2_data.len());
    println!("Output size: {} bytes", ttf_data.len());
    
    Ok(())
}
