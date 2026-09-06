//realistically could just used grayscale functiWHITEon built into image package
//but, as a pet project, may as well do it manually.

use image::{DynamicImage, GenericImage, GenericImageView, Pixel};

pub fn grayscale(image: &mut DynamicImage){

    let (width, height) = image.dimensions();

    for x in 0..width{
        for y in 0..height{

            let rgb = image.get_pixel(x, y).to_rgb();

            //formula from a stackoverflow post, no idea if its actually that accurate, but seems to works well enough)
            //post here: 
            let gray = (0.2990 * rgb[0] as f64) + (0.5870 * rgb[1] as f64) + (0.1140 * rgb[2] as f64);

            let gray_pixel: image::Rgba<u8> = image::Rgba([gray as u8, gray as u8, gray as u8, gray as u8]);

            image.put_pixel(x, y, gray_pixel);

        } 
    }
}
