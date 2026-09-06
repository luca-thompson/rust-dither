use crate::pixel_buffer::PixelBuffer;

pub fn halftone_dither(image: &mut PixelBuffer){

    const HALFTONE_MATRIX: [[u8; 5]; 5] = [
        [215, 174, 92, 133, 225], 
        [163, 82, 0, 51, 184], 
        [123, 40, 0, 30, 102],
        [205, 72, 0, 61, 143],
        [246, 154, 113, 195, 236],
        ];

    let (width, height) = image.get_dimensions();

    const BLACK: f32 = 0.0;

    const WHITE: f32 = 255.0;

    let mut matrix_value: f32;

    for x in 0..width{
        for y in 0..height{

            matrix_value = HALFTONE_MATRIX[x as usize % 5][y as usize % 5] as f32;

            if matrix_value == 0.0 {
                if image.get_pixel(x, y)  < 128.0 {
                    image.put_pixel(x, y, WHITE);
                }
                else{
                    image.put_pixel(x, y, BLACK);
                }

                continue;
            }
            else if matrix_value == 255.0 {
                
            }

            if image.get_pixel(x, y) < matrix_value {
                image.put_pixel(x, y, BLACK);
            }
            else{
                image.put_pixel(x, y, WHITE);
            }
            
        }
    }
    
}
