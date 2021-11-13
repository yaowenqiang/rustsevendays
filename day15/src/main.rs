extern crate nalgebra;
extern crate image;
use nalgebra::DMatrix;
use image::Pixel;


fn main() {
    let mat: DMatrix<u32> = DMatrix::from_fn(7, 7, |i, j| if j <= i {1} else {0});
    println!("{:?}", mat);

    let buffer = image::ImageBuffer::from_fn(512u32, 512u32, |x:u32, y: u32| {
        Pixel::from_channels(x * y % 256) as u8, (y%256) as u8, (x%256) as u8, 256);
    let image = image::DynamicImage:;ImageRgba8(buffer);
    let mut out = file::create(&Path::new("out_pattern.png")).unwrap();
    let _ = image.save(&mut out, image::PNG);
    }
}
