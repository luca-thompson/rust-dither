#[path = "../src/pixel_buffer.rs"] mod pixel_buffer;
use pixel_buffer::PixelBuffer;

#[path = "../src/dither/bayer_2.rs"] mod bayer_2;

#[test]

fn test_bayer_2() {
    
    let test_image_vec: Vec<f32> = vec![
        0.0, 127.0, 
        191.0, 63.0,
        1.0, 129.0, 
        193.0, 65.0
    ];
    let mut test_pixel_buffer: PixelBuffer = PixelBuffer{width: 2, height: 4, pixels: test_image_vec};

    let real_image_vec: Vec<f32> = vec![
        0.0, 0.0, 
        0.0, 0.0,  
        255.0, 255.0, 
        255.0, 255.0
    ];
    let real_pixel_buffer: PixelBuffer = PixelBuffer{width: 2, height: 4, pixels: real_image_vec};
    
    bayer_2::bayer_dither(&mut test_pixel_buffer);

    assert_eq!(test_pixel_buffer.width, real_pixel_buffer.width);
    assert_eq!(test_pixel_buffer.height, real_pixel_buffer.height);
    assert_eq!(test_pixel_buffer.pixels, real_pixel_buffer.pixels);
}
