use std::env;
use std::fs;
use std::io::{self, Read, Write};
use woff2_sys::convert_ttf_to_woff2;

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if we have the correct number of arguments
    if args.len() != 3 {
        eprintln!("Usage: {} <input.ttf> <output.woff2>", args[0]);
        return Ok(());
    }
    
    let input_path = &args[1];
    let output_path = &args[2];
    
    // Read the TTF file
    println!("Reading TTF file: {}", input_path);
    let mut ttf_data = Vec::new();
    let mut file = fs::File::open(input_path)?;
    file.read_to_end(&mut ttf_data)?;
    
    // Convert TTF to WOFF2
    println!("Converting TTF to WOFF2...");
    let woff2_data = match convert_ttf_to_woff2(&ttf_data, &[], 11, true) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error: Failed to convert TTF to WOFF2");
            return Err(io::Error::new(io::ErrorKind::Other, "Conversion failed"));
        }
    };
    
    // Write the WOFF2 file
    println!("Writing WOFF2 file: {}", output_path);
    let mut output_file = fs::File::create(output_path)?;
    output_file.write_all(&woff2_data)?;
    
    println!("Conversion successful!");
    println!("Input size: {} bytes", ttf_data.len());
    println!("Output size: {} bytes", woff2_data.len());
    println!("Compression ratio: {:.2}%", (woff2_data.len() as f64 / ttf_data.len() as f64) * 100.0);
    
    Ok(())
}
