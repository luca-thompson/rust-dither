//args
use clap::{Parser};
#[derive(Parser, Debug)]
struct Args {

    #[arg(long)]
    f_in: String,

    #[arg(long)]
    f_out: String,

    #[arg(long)]
    algorithm: String

}

#[path = "dispatch.rs"] mod dispatch;
mod pixel_buffer;
use pixel_buffer::PixelBuffer;

//image
use image::{ImageReader};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    println!("File in: '{}'.", args.f_in);

    let mut image = ImageReader::open(args.f_in)?.decode()?;
    let mut pixel_buffer: PixelBuffer = PixelBuffer::to_pixel_buffer(&image);

    println!("Dithering using '{}' algorithm.", args.algorithm);
    
    dispatch::dispatch(&mut pixel_buffer, args.algorithm);
    image = PixelBuffer::to_dynamic_image(pixel_buffer);
    
    println!("Success! Saving as: '{}'.", args.f_out);

    image.save(args.f_out).unwrap();

    Ok(())
}
