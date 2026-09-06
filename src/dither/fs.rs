use crate::pixel_buffer::PixelBuffer;

pub fn fs_dither(image: &mut PixelBuffer) {

    let (width, height) = image.get_dimensions();

    const BLACK: f32 = 0.0;
    const WHITE: f32 = 255.0;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            
            let old_pixel_value: f32 = image.get_pixel(x, y);
            let new_pixel_value: f32;

            if old_pixel_value < 128.0 {
                image.put_pixel(x, y, BLACK);
                new_pixel_value = BLACK;
            } else {
                image.put_pixel(x, y, WHITE);
                new_pixel_value = WHITE;
            }

            let quant_error: f32 = old_pixel_value - new_pixel_value;

            let mut old_val = image.get_pixel(x + 1, y);
            image.put_pixel(x + 1, y, diffuse(old_val, &quant_error, 7));

            old_val = image.get_pixel(x - 1, y+1);
            image.put_pixel(x - 1, y + 1, diffuse(old_val, &quant_error, 3));

            old_val = image.get_pixel(x, y+1);
            image.put_pixel(x, y + 1, diffuse(old_val, &quant_error, 5));

            old_val = image.get_pixel(x+1, y+1);
            image.put_pixel(x + 1, y + 1, diffuse(old_val, &quant_error, 1));

        }
    }
}

fn diffuse(old: f32, quant_error: &f32, numerator: u32) -> f32 {
    
    let offset: f32 = (quant_error * numerator as f32) / 16.0;

    let new_value = (old + offset).clamp(0.0, 255.0) as i32 as f32;

    new_value
}
