use std::io;
use std::{fs::File, io::Read};

#[derive(Debug)]
struct BMPHeader {
    magic: u16,            // Magic identifier: 0x4d42
    size: u32,             // File size in bytes
    reserved1: u16,        // Not used
    reserved2: u16,        // Not used
    offset: u32,           // Offset to image data in bytes from beginning of file
    dib_header_size: u32,  // DIB Header size in bytes
    width_px: i32,         // Width of the image
    height_px: i32,        // Height of image
    num_planes: u16,       // Number of color planes
    bits_per_pixel: u16,   // Bits per pixel
    compression: u32,      // Compression type
    image_size_bytes: u32, // Image size in bytes
    x_resolution_ppm: i32, // Pixels per meter
    y_resolution_ppm: i32, // Pixels per meter
    num_colors: u32,       // Number of colors
    important_colors: u32, // Important colors
}

fn main() -> io::Result<()> {
    let filename = "6x6_24bit.bmp";

    let loaded_bmp: BMPHeader;

    match File::open(filename) {
        Ok(mut file) => {
            let mut magic_bytes = [0; 2];
            file.read_exact(&mut magic_bytes)?;
            let mut size_bytes = [0; 4];
            file.read_exact(&mut size_bytes)?;
            let mut reserved1_bytes = [0; 2];
            file.read_exact(&mut reserved1_bytes)?;
            let mut reserved2_bytes = [0; 2];
            file.read_exact(&mut reserved2_bytes)?;
            let mut offset_bytes = [0; 4];
            file.read_exact(&mut offset_bytes)?;
            let mut dib_header_size_bytes = [0; 4];
            file.read_exact(&mut dib_header_size_bytes)?;
            let mut width_px_bytes = [0; 4];
            file.read_exact(&mut width_px_bytes)?;
            let mut height_px_bytes = [0; 4];
            file.read_exact(&mut height_px_bytes)?;
            let mut num_planes_bytes = [0; 2];
            file.read_exact(&mut num_planes_bytes)?;
            let mut bits_per_pixel_bytes = [0; 2];
            file.read_exact(&mut bits_per_pixel_bytes)?;
            let mut compression_bytes = [0; 4];
            file.read_exact(&mut compression_bytes)?;
            let mut image_size_bytes_val = [0; 4];
            file.read_exact(&mut image_size_bytes_val)?;
            let mut x_resolution_ppm_bytes = [0; 4];
            file.read_exact(&mut x_resolution_ppm_bytes)?;
            let mut y_resolution_ppm_bytes = [0; 4];
            file.read_exact(&mut y_resolution_ppm_bytes)?;
            let mut num_colors_bytes = [0; 4];
            file.read_exact(&mut num_colors_bytes)?;
            let mut important_colors_bytes = [0; 4];
            file.read_exact(&mut important_colors_bytes)?;

            loaded_bmp = BMPHeader {
                magic: u16::from_le_bytes(magic_bytes),
                size: u32::from_le_bytes(size_bytes),
                reserved1: u16::from_le_bytes(reserved1_bytes),
                reserved2: u16::from_le_bytes(reserved2_bytes),
                offset: u32::from_le_bytes(offset_bytes),
                dib_header_size: u32::from_le_bytes(dib_header_size_bytes),
                width_px: i32::from_le_bytes(width_px_bytes),
                height_px: i32::from_le_bytes(height_px_bytes),
                num_planes: u16::from_le_bytes(num_planes_bytes),
                bits_per_pixel: u16::from_le_bytes(bits_per_pixel_bytes),
                compression: u32::from_le_bytes(compression_bytes),
                image_size_bytes: u32::from_le_bytes(image_size_bytes_val),
                x_resolution_ppm: i32::from_le_bytes(x_resolution_ppm_bytes),
                y_resolution_ppm: i32::from_le_bytes(y_resolution_ppm_bytes),
                num_colors: u32::from_le_bytes(num_colors_bytes),
                important_colors: u32::from_le_bytes(important_colors_bytes),
            };

            println!("--- BMP Header ---");
            println!("Magic Number: {:X}", loaded_bmp.magic);
            println!("File Size: {} bytes", loaded_bmp.size);
            println!("Reserved1: {}", loaded_bmp.reserved1);
            println!("Reserved2: {}", loaded_bmp.reserved2);
            println!("Image Data Offset: {} bytes", loaded_bmp.offset);
            println!("DIB Header Size: {} bytes", loaded_bmp.dib_header_size);
            println!("Width: {} px", loaded_bmp.width_px);
            println!("Height: {} px", loaded_bmp.height_px);
            println!("Number of Color Planes: {}", loaded_bmp.num_planes);
            println!("Bits Per Pixel: {}", loaded_bmp.bits_per_pixel);
            println!("Compression Type: {}", loaded_bmp.compression);
            println!("Image Size: {} bytes", loaded_bmp.image_size_bytes);
            println!("X Pixels Per Meter: {}", loaded_bmp.x_resolution_ppm);
            println!("Y Pixels Per Meter: {}", loaded_bmp.y_resolution_ppm);
            println!("Number of Colors in Palette: {}", loaded_bmp.num_colors);
            println!(
                "Number of Important Colors: {}",
                loaded_bmp.important_colors
            );

            // println!("\nFull Header Struct:\n{:#?}", loaded_bmp);
        }
        Err(e) => {
            eprintln!("File not found {}, {}", filename, e);
        }
    }
    Ok(())
}
