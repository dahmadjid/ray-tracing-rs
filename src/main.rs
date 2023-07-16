mod vec3;
mod ray;
mod image;

use ray::Ray;
use vec3::Vec3;
use image::Image;

fn ray_color(ray: &Ray<f64>) -> Vec3<f64> {
    if let Some(solution) = hit_sphere(&Vec3::<f64>::new(0.,0.,-1.), 0.5f64, ray) {
        let normal = (ray.at(solution) - Vec3::<f64>::new(0., 0., -1.)).normalize();
        Vec3::new(normal.x + 1., normal.y + 1., normal.z + 1.).scale(0.5)
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5*(unit_direction.y + 1.0);
        Vec3::new(1., 1., 1.).scale(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scale(t)
    }
}


fn hit_sphere(center: &Vec3<f64>, radius: f64, r: &Ray<f64>) -> Option<f64> {
    let oc = r.origin.clone() - center.clone();
    let a = r.direction.dot(r.direction.clone());
    let b = oc.dot(r.direction.clone()) * 2.0;
    let c = oc.dot(oc.clone()) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt() ) / (2.0*a))
    }
}
fn main() {
    let aspect_ratio = 16 as f64/9 as f64;
    let mut img = Image::new(1024, (1024. / aspect_ratio) as u32);

    let viewport_height = 2.00f64;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.00f64;
    let origin = Vec3::<f64>::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin.clone() - horizontal.scale(0.5) - vertical.scale(0.5) - Vec3::new(0., 0., focal_length);


    for j in (0..img.height).rev() {
        for i in 0..img.width {
            let u = i as f64 / (img.width  -1) as f64;
            let v = j as f64 / (img.height -1) as f64;
            let ray = Ray{origin:origin.clone(), direction:lower_left_corner.clone() + horizontal.scale(u) + vertical.scale(v) - origin.clone()};
            img.data.push(ray_color(&ray).scale(255.999).floor());
        }
    }

    img.render("img.ppm");
    img.show(); 
}
