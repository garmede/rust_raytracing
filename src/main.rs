use std::io::{self, Write};

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {j}");
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let r = i as f64 / image_width as f64;
            let g = j as f64 / image_height as f64;
            let b = 0.25;
            
            let ir = (r * 256.0) as u8;
            let ig = (g * 256.0) as u8;
            let ib = (b * 256.0) as u8;

            let pixel = imgbuf.get_pixel_mut(i, image_height - 1 - j);
            *pixel = image::Rgb([ir, ig, ib]);
        }
    }

    imgbuf.save("render/first_image.png").unwrap();
    print!("\nDone.\n");
}
