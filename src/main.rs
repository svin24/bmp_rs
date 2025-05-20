use std::io;
use std::{fs::File, io::Read};

const BMP_MAGIC: u16 = 0x4D42; // Big endian we convert to little endian later

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

// It is apparently recommended I make handlers
fn read_u16_le(reader: &mut impl Read) -> io::Result<u16> {
    let mut buffer = [0u8; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

fn read_u32_le(reader: &mut impl Read) -> io::Result<u32> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_i32_le(reader: &mut impl Read) -> io::Result<i32> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
}

fn main() -> io::Result<()> {
    let filename = "6x6_24bit.bmp";

    let loaded_bmp: BMPHeader;

    match File::open(filename) {
        Ok(mut file) => {
            loaded_bmp = BMPHeader {
                magic: {
                    let magic_val = read_u16_le(&mut file)?;
                    if magic_val != BMP_MAGIC {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a BMP file"));
                    }
                    magic_val
                },
                size: read_u32_le(&mut file)?,
                reserved1: read_u16_le(&mut file)?,
                reserved2: read_u16_le(&mut file)?,
                offset: read_u32_le(&mut file)?,
                dib_header_size: read_u32_le(&mut file)?,
                width_px: read_i32_le(&mut file)?,
                height_px: read_i32_le(&mut file)?,
                num_planes: read_u16_le(&mut file)?,
                bits_per_pixel: read_u16_le(&mut file)?,
                compression: read_u32_le(&mut file)?,
                image_size_bytes: read_u32_le(&mut file)?,
                x_resolution_ppm: read_i32_le(&mut file)?,
                y_resolution_ppm: read_i32_le(&mut file)?,
                num_colors: read_u32_le(&mut file)?,
                important_colors: read_u32_le(&mut file)?,
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
