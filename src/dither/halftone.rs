use crate::pixel_buffer::PixelBuffer;

pub fn halftone_dither(image: &mut PixelBuffer){

    const HALFTONE_MATRIX: [[u8; 10]; 10] = [
        [214, 173, 92, 134, 224, 0, 0, 0, 0, 0], 
        [153, 82, 10, 51, 194, 0, 0, 0, 0, 0], 
        [122, 40, 0, 20, 102, 0, 0, 0, 0, 0],
        [204, 71, 31, 61, 163, 0, 0, 0, 0, 0],
        [245, 143, 112, 184, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 245, 143, 112, 184, 235],
        [0, 0, 0, 0, 0, 204, 71, 31, 61, 163],
        [0, 0, 0, 0, 0, 122, 40, 0, 20, 102],
        [0, 0, 0, 0, 0, 153, 82, 10, 51, 194],
        [0, 0, 0, 0, 0, 214, 173, 92, 134, 224],
        ];

    let (width, height) = image.get_dimensions();

    const BLACK: f32 = 0.0;

    const WHITE: f32 = 255.0;

    let mut matrix_value: f32;

    for x in 0..width{
        for y in 0..height{

            matrix_value = HALFTONE_MATRIX[x as usize % 10][y as usize % 10] as f32;

            if matrix_value == 0.0 {
                if image.get_pixel(x, y)  < 128.0 {
                    image.put_pixel(x, y, WHITE); 
                }
                else{
                    image.put_pixel(x, y, BLACK);
                }

                continue;
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
