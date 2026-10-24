#[path = "../src/pixel_buffer.rs"] mod pixel_buffer;
use pixel_buffer::PixelBuffer;

#[path = "../src/dither/bayer_4.rs"] mod bayer_4;

#[test]

fn test_bayer_4() {
    
    let test_image_vec: Vec<f32> = vec![
        0.0, 191.0, 47.0, 239.0,
        127.0, 63.0, 175.0, 111.0,
        31.0, 223.0, 15.0, 207.0, 
        159.0, 95.0, 143.0, 79.0,
        
        1.0, 193.0, 49.0, 241.0,
        129.0, 65.0, 177.0, 113.0,
        33.0, 225.0, 17.0, 209.0, 
        161.0, 97.0, 145.0, 81.0
    ];
    let mut test_pixel_buffer: PixelBuffer = PixelBuffer{width: 4, height: 8, pixels: test_image_vec};

    let real_image_vec: Vec<f32> = vec![
        0.0, 0.0, 0.0, 0.0, 
        0.0, 0.0, 0.0, 0.0, 
        0.0, 0.0, 0.0, 0.0, 
        0.0, 0.0, 0.0, 0.0, 
        
        255.0, 255.0, 255.0, 255.0, 
        255.0, 255.0, 255.0, 255.0, 
        255.0, 255.0, 255.0, 255.0, 
        255.0, 255.0, 255.0, 255.0
    ];
    let real_pixel_buffer: PixelBuffer = PixelBuffer{width: 4, height: 8, pixels: real_image_vec};
    
    bayer_4::bayer_dither(&mut test_pixel_buffer);

    assert_eq!(test_pixel_buffer.width, real_pixel_buffer.width);
    assert_eq!(test_pixel_buffer.height, real_pixel_buffer.height);
    assert_eq!(test_pixel_buffer.pixels, real_pixel_buffer.pixels);
}
