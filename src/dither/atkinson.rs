use crate::pixel_buffer::PixelBuffer;

pub fn atkinson_dither(image: &mut PixelBuffer) {

    let (width, height) = image.get_dimensions();

    const BLACK: f32 = 0.0;
    const WHITE: f32 = 255.0;

    for x in 1..width - 2 {
        for y in 1..height - 2 {
            
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
            image.put_pixel(x + 1, y, diffuse(old_val, &quant_error));

            old_val = image.get_pixel(x + 2, y);
            image.put_pixel(x + 2, y, diffuse(old_val, &quant_error));

            old_val = image.get_pixel(x - 1, y + 1);
            image.put_pixel(x - 1, y + 1, diffuse(old_val, &quant_error));
            
            old_val = image.get_pixel(x, y + 1);
            image.put_pixel(x, y + 1, diffuse(old_val, &quant_error));
            
            old_val = image.get_pixel(x+1, y+1);
            image.put_pixel(x + 1, y + 1, diffuse(old_val, &quant_error));

            old_val = image.get_pixel(x, y + 2);
            image.put_pixel(x, y + 2, diffuse(old_val, &quant_error));

        }
    }
}

fn diffuse(old: f32, quant_error: &f32,) -> f32 {
    
    let offset: f32 = quant_error / 8.0;

    let new_value = (old + offset).clamp(0.0, 255.0) as i32 as f32;

    new_value
}
