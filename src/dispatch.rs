#[path = "dither/threshold.rs"] mod threshold;
#[path = "dither/bayer_2.rs"] mod bayer_2;
#[path = "dither/bayer_4.rs"] mod bayer_4;
#[path = "dither/halftone.rs"] mod halftone;
#[path = "dither/random.rs"] mod random;
#[path = "dither/fs.rs"] mod fs;
#[path = "dither/atkinson.rs"] mod atkinson;

use crate::pixel_buffer::PixelBuffer;


pub fn dispatch(pixel_buffer: &mut PixelBuffer, algorithm: String){

    let alg = algorithm.as_str();

    match alg {
        "grayscale" => {
            println!("TODO: IMPLEMENT GRAYSCALE-ONLY")
        }
        "threshold" => {
            threshold::threshold_dither(pixel_buffer);
        }
        "bayer_2" => {
            bayer_2::bayer_dither(pixel_buffer);
        }
        "bayer_4" => {
            bayer_4::bayer_dither(pixel_buffer);
        }
        "halftone" => {
            halftone::halftone_dither(pixel_buffer);
        }
        "random" => {
            random::random_dither(pixel_buffer);
        }
        "floyd_steinberg" => {
            fs::fs_dither(pixel_buffer);
        }
        "atkinson" => {
            atkinson::atkinson_dither(pixel_buffer);
        }
        _ => {
            println!("Unrecognised algorithm: '{}'.", algorithm);
        }
    }
}
