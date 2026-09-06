use rand::RngExt;

use crate::pixel_buffer::PixelBuffer;

pub fn random_dither(image: &mut PixelBuffer){

    let (width, height) = image.get_dimensions();

    for x in 0..width {
        for y in 0..height {
            if image.get_pixel(x, y) < rand::rng().random_range(0..255) as f32 {
                image.put_pixel(x, y, 0.0);
            } else {
                image.put_pixel(x, y, 255.0);
            }
        }
    }
}
