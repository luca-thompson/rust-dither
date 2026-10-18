#[path = "../src/pixel_buffer.rs"] mod pixel_buffer;
use pixel_buffer::PixelBuffer;

#[path = "../src/dither/threshold.rs"] mod threshold;

#[test]

fn test_threshold() {
    
    let test_image_vec: Vec<f32> = vec![0.0, 28.0, 56.0, 84.0, 112.0, 144.0, 168.0, 196.0, 224.0,];
    let mut test_pixel_buffer: PixelBuffer = PixelBuffer{width: 3, height: 3, pixels: test_image_vec};

    let real_image_vec: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 255.0, 255.0, 255.0, 255.0,];
    let real_pixel_buffer: PixelBuffer = PixelBuffer{width: 3, height: 3, pixels: real_image_vec};
    
    threshold::threshold_dither(&mut test_pixel_buffer);

    assert_eq!(test_pixel_buffer.width, real_pixel_buffer.width);
    assert_eq!(test_pixel_buffer.height, real_pixel_buffer.height);
    assert_eq!(test_pixel_buffer.pixels, real_pixel_buffer.pixels);
}
