fn main() {
    // Image

    let imgx = 256;
    let imgy = 256;

    // Render
    
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (x as f32) as u8;
        let g = (256.0 - y as f32) as u8;
        *pixel = image::Rgb([r, g, 64]);
    }

    imgbuf.save("render/first_image.png").unwrap();
}
