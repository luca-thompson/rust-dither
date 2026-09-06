use image::{DynamicImage, GenericImageView, ImageBuffer};

pub struct PixelBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<f32>
}

impl PixelBuffer {
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    
    pub fn get_pixel(&self, x: u32, y: u32 ) -> f32 {
        let index = (y * self.width + x) as usize;
        self.pixels[index]
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, value: f32 ) {
        let index = (y * self.width + x) as usize;
        self.pixels[index] = value;
    }

    pub fn to_pixel_buffer(img: &DynamicImage) -> PixelBuffer {
        
        let (width, height) = img.dimensions();
        let pixels: Vec<f32> = img.to_luma8().into_raw().iter().map(|&p| p as f32).collect();
        
        PixelBuffer {
            width: width,
            height: height,
            pixels,
        }
    }

    pub fn to_dynamic_image(img: PixelBuffer) -> DynamicImage {

        let bytes = img.pixels.iter().map(|&p| p as u8).collect();
        let img_from_raw = ImageBuffer::from_vec(img.width, img.height, bytes).expect("must be correct format");

        DynamicImage::ImageLuma8(img_from_raw)
    }
}
