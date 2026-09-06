use crate::pixel_buffer::PixelBuffer;

pub fn bayer_dither(image: &mut PixelBuffer){

    const BAYER_MATRIX_TWO: [[u8; 2]; 2] = [[0, 128], [192, 64]];

    let (width, height) = image.get_dimensions();

    for x in 0..width{
        for y in 0..height{
            if image.get_pixel(x, y) < BAYER_MATRIX_TWO[x as usize % 2][y as usize % 2] as f32 {
                image.put_pixel(x, y, 0.0);
            }
            else{
                image.put_pixel(x, y, 255.0);
            }
            
        }
    }
    
}
