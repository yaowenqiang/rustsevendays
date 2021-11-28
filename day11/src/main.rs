extern crate image;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

//use std::fs::File;

fn main() {
    //let img = image::open("data/in.png").expect("Opening image failed");
    //let filtered = img.fliph();
    //let mut out = File::create("out.png").unwrap();
    //filtered.save(&mut out, image::PNG).expect("Saving image failed");
    //

    //let img: RgbImage = ImageBuffer::new(512,512);

    let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let (width, height) = img.dimensions();
    println!("image width : {}, image height: {}", width, height);

    let pixel = img[(100, 100)];

    img.put_pixel(100, 100, pixel);

    for pixel in img.pixels() {
        //println!("{}", pixel);
    }

    img.save("test.png").unwrap();
}
