#[path = "../src/pixel_buffer.rs"] mod pixel_buffer;
use pixel_buffer::PixelBuffer;

//image
use image::ImageReader;


#[test]

fn test_io() {
    let test_image_vec: Vec<f32> = vec![0.0, 255.0, 255.0, 0.0];
    let test_pixel_buffer: PixelBuffer = PixelBuffer{width: 2, height: 2, pixels: test_image_vec};

    let image = ImageReader::open("tests/test_image_bw_2x2.png").unwrap().decode().unwrap();
    
    let real_pixel_buffer: PixelBuffer = PixelBuffer::to_pixel_buffer(&image);

    assert_eq!(test_pixel_buffer.width, real_pixel_buffer.width);
    assert_eq!(test_pixel_buffer.height, real_pixel_buffer.height);
    assert_eq!(test_pixel_buffer.pixels, real_pixel_buffer.pixels);
}
