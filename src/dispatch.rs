#[path = "dither/threshold.rs"] mod threshold;
#[path = "dither/bayer_2.rs"] mod bayer_2;
#[path = "dither/bayer_4.rs"] mod bayer_4;
#[path = "dither/halftone.rs"] mod halftone;
#[path = "dither/random.rs"] mod random;
#[path = "dither/fs.rs"] mod fs;
#[path = "grayscale.rs"] mod grayscale;

use image::DynamicImage;


pub fn dispatch(image: &mut DynamicImage, algorithm: String){

    let alg = algorithm.as_str();

    match alg {
        "grayscale" => {
            grayscale::grayscale(image);
        }
        "threshold" => {
            threshold::threshold_dither(image);
        }
        "bayer_2" => {
            bayer_2::bayer_dither(image);
        }
        "bayer_4" => {
            bayer_4::bayer_dither(image);
        }
        "halftone" => {
            halftone::halftone_dither(image);
        }
        "random" => {
            random::random_dither(image);
        }
        "floyd_steinberg" => {
            fs::fs_dither(image);
        }
        _ => {
            println!("Unrecognised algorithm: '{}'.", algorithm);
        }
    }
}