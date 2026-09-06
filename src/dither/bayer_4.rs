use crate::pixel_buffer::PixelBuffer;

pub fn bayer_dither(image: &mut PixelBuffer){

    const BAYER_MATRIX_FOUR: [[u8; 4]; 4] = [
        [0, 192, 48, 240], 
        [128, 64, 176, 112], 
        [32, 224, 16, 208], 
        [160, 96, 144, 80]
    ];

    let (width, height) = image.get_dimensions();

    for x in 0..width{
        for y in 0..height{
            if image.get_pixel(x, y) < BAYER_MATRIX_FOUR[x as usize % 4][y as usize % 4] as f32 {
                image.put_pixel(x, y, 0.0);
            }
            else{
                image.put_pixel(x, y, 255.0);
            }
            
        }
    }
    
}
