use std::io;
use std::{fs::File, io::Read};

const BMP_MAGIC: u16 = 0x4D42; // Big endian we convert to little endian later

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

impl BMPHeader {
    pub fn header_read(reader: &mut impl Read) -> io::Result<Self> {
        let magic_val = {
            let val = read_u16_le(reader)?;
            if val != BMP_MAGIC {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Not a BMP file. Expected magic 0x{:X}, got 0x{:X}",
                        BMP_MAGIC, val
                    ),
                ));
            }
            val
        };

        Ok(BMPHeader {
            magic: magic_val,
            size: read_u32_le(reader)?,
            reserved1: read_u16_le(reader)?,
            reserved2: read_u16_le(reader)?,
            offset: read_u32_le(reader)?,
            dib_header_size: read_u32_le(reader)?,
            width_px: read_i32_le(reader)?,
            height_px: read_i32_le(reader)?,
            num_planes: read_u16_le(reader)?,
            bits_per_pixel: read_u16_le(reader)?,
            compression: read_u32_le(reader)?,
            image_size_bytes: read_u32_le(reader)?,
            x_resolution_ppm: read_i32_le(reader)?,
            y_resolution_ppm: read_i32_le(reader)?,
            num_colors: read_u32_le(reader)?,
            important_colors: read_u32_le(reader)?,
        })
    }
}

fn main() -> io::Result<()> {
    let filename = "6x6_24bit.bmp";

    match File::open(filename) {
        Ok(mut file) => {
            let header_bmp = BMPHeader::header_read(&mut file)?;
            println!("Successfully loaded BMP image!");
            println!("--- Header ---");
            println!("Magic: {:X}", header_bmp.magic);
            println!("File Size: {}", header_bmp.size);
            println!("Reserved1: {}", header_bmp.reserved1);
            println!("Reserved2: {}", header_bmp.reserved2);
            println!("Pixel Data Offset: {}", header_bmp.offset);
            println!("DIB Header Size: {}", header_bmp.dib_header_size);
            println!("Number of Color Planes: {}", header_bmp.num_planes);
            println!("Width: {}px", header_bmp.width_px);
            println!("Height: {}px", header_bmp.height_px);
            println!("Bits Per Pixel: {}", header_bmp.bits_per_pixel);
            println!("Compression: {}", header_bmp.compression);
            println!(
                "Image Size (from header): {} bytes",
                header_bmp.image_size_bytes
            );
            println!("X Pixels Per Meter: {}", header_bmp.x_resolution_ppm);
            println!("Y Pixels Per Meter: {}", header_bmp.y_resolution_ppm);
            println!("Number of Colors in Palette: {}", header_bmp.num_colors);
            println!(
                "Number of Important Colors: {}",
                header_bmp.important_colors
            );
            //println!("\nFull Header Struct:\n{:#?}", header_bmp);
        }
        Err(e) => {
            eprintln!("File not found {}, {}", filename, e);
        }
    }
    Ok(())
}
