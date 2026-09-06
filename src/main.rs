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
mod image_buffer;
use image_buffer::ImageBuffer;

//image
use image::{DynamicImage, GenericImageView, ImageReader,};




pub fn to_image_buffer(img: &DynamicImage) -> ImageBuffer {
    
    let (width, height) = img.dimensions();
    let pixels: Vec<f32> = img.to_luma8().into_raw().iter().map(|&p| p as f32).collect();
    
    ImageBuffer {
        width: width,
        height: height,
        pixels,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    println!("File in: '{}'.", args.f_in);

    let mut img = ImageReader::open(args.f_in)?.decode()?;

    let mut img_buff = to_image_buffer(&img);

    println!("Dithering using '{}' algorithm.", args.algorithm);
    dispatch::dispatch(&mut img_buf, args.algorithm);

    println!("Success! Saving as: '{}'.", args.f_out);

    let _ = img.save(args.f_out);


    Ok(())
}
