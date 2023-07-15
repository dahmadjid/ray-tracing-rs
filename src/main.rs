mod vec3;
mod ray;
mod image;

use vec3::Vec3;
use image::Image;

fn main() {
    let aspect_ratio = 16 as f64/9 as f64;
    let mut img = Image::new(1024, (1024. / aspect_ratio) as u32);



    for j in (0..img.height).rev() {
        for i in (0..img.width).rev() {
            let pixel = Vec3::new(
                j as f64 / (img.width  -1) as f64,
                i as f64 / (img.height -1) as f64,
                0.25
            ).scale(255.999).floor();
            img.data.push(pixel);
        }
    }
    img.render("img.ppm");
    img.show(); 

        
}
