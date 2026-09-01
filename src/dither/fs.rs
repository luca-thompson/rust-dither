#[path = "../grayscale.rs"]
mod grayscale;

use image::{DynamicImage, GenericImage, GenericImageView};

pub fn fs_dither(image: &mut DynamicImage) {
    grayscale::grayscale(image);

    let (width, height) = image.dimensions();

    const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);

    const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            
            let old_pixel_value: u8 = image.get_pixel(x, y)[0];
            let new_pixel_value: i32;

            if old_pixel_value < 128 {
                image.put_pixel(x, y, BLACK);
                new_pixel_value = 0
            } else {
                image.put_pixel(x, y, WHITE);
                new_pixel_value = 255
            }

            let quant_error: i32 = (old_pixel_value as i32) - new_pixel_value;

            let mut old_val = image.get_pixel(x + 1, y)[0];
            image.put_pixel(x + 1, y, diffuse(old_val, &quant_error, 7));

            old_val = image.get_pixel(x - 1, y+1)[0];
            image.put_pixel(x - 1, y + 1, diffuse(old_val, &quant_error, 3));

            old_val = image.get_pixel(x, y+1)[0];
            image.put_pixel(x, y + 1, diffuse(old_val, &quant_error, 5));

            old_val = image.get_pixel(x+1, y+1)[0];
            image.put_pixel(x + 1, y + 1, diffuse(old_val, &quant_error, 1));

        }
    }
}

fn diffuse(old: u8, quant_error: &i32, numerator: i32) -> image::Rgba<u8> {
    
    let offset: i32 = (quant_error * numerator) / 16;

    let new_value = (old as i32 + offset).clamp(0, 255) as u8;

    return image::Rgba([new_value, new_value, new_value, 255]);
}
