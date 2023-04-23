use raytracer::{
    color::Color,
    geometry::sphere::Sphere,
    util::print_color,
    vec::Vector3,
    view::ray::{Hit, Ray},
};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Vector3::zero();
    let horizontal = Vector3::new(viewport_width, 0, 0);
    let vertical = Vector3::new(0, viewport_height, 0);
    let lower_left_corner =
        origin - horizontal / 2 - vertical / 2 - Vector3::new(0, 0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = Ray::of(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let sphere = Sphere::new(Vector3::new(0, 0, -1), 1.);

            let sphere_hit = sphere.hit(&ray, (0., f64::INFINITY));
            let pixel_color = match sphere_hit {
                None => {
                    let normalized_direction = ray.direction().normalize();
                    let factor = 0.5 * (normalized_direction.y() + 1.);
                    Vector3::lerp(&Color::new(1, 1, 1), &Color::new(0.5, 0.7, 1), factor)
                }
                Some(ray_hit) => ray_hit.normal + Vector3::new(1, 1, 1),
            };
            print_color(pixel_color);
        }
    }
}
