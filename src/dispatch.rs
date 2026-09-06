#[path = "dither/threshold.rs"] mod threshold;
#[path = "dither/bayer_2.rs"] mod bayer_2;
#[path = "dither/bayer_4.rs"] mod bayer_4;
#[path = "dither/halftone.rs"] mod halftone;
#[path = "dither/random.rs"] mod random;
#[path = "dither/fs.rs"] mod fs;
#[path = "dither/atkinson.rs"] mod atkinson;
#[path = "grayscale.rs"] mod grayscale;

mod image_buffer;
use image_buffer::ImageBuffer;


pub fn dispatch(img_buff: &mut ImageBuffer, algorithm: String){

    let alg = algorithm.as_str();

    match alg {
        "grayscale" => {
            grayscale::grayscale(img_buff);
        }
        "threshold" => {
            threshold::threshold_dither(img_buff);
        }
        "bayer_2" => {
            bayer_2::bayer_dither(img_buff);
        }
        "bayer_4" => {
            bayer_4::bayer_dither(img_buff);
        }
        "halftone" => {
            halftone::halftone_dither(img_buff);
        }
        "random" => {
            random::random_dither(img_buff);
        }
        "floyd_steinberg" => {
            fs::fs_dither(img_buff);
        }
        "atkinson" => {
            atkinson::atkinson_dither(img_buff);
        }
        _ => {
            println!("Unrecognised algorithm: '{}'.", algorithm);
        }
    }
}
