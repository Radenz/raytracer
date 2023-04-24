use std::{sync::Arc, time::Instant};

use image::{ImageBuffer, Rgb};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use raytracer::{
    object::{
        geometry::{sphere::Sphere, vector::Vector3},
        material::{
            color::Color, dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material,
        },
    },
    render::pixel::Vector3Extension,
    util::{random::Random, ray_color_diffuse},
    vec3,
    view::{camera::Camera, ray::HitTarget},
};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples = 500;
    let max_depth = 50;

    let world = random_scene();
    let camera = Camera::new(
        vec3!(13, 2, 3),
        Vector3::zero(),
        Vector3::up(),
        20.,
        aspect_ratio,
        0.1,
        10.,
    );

    let start = Instant::now();

    let mut buffer = ImageBuffer::new(image_width, image_height);
    buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .into_par_iter()
        .for_each(|(i, j, pixel)| {
            let mut color_sum = Vector3::zero();
            for _ in 0..samples {
                let u = (i as f64 + Random::f64()) / (image_width - 1) as f64;
                let v = ((image_height - 1 - j) as f64 + Random::f64()) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                color_sum += ray_color_diffuse(&ray, &world, max_depth);
            }

            let sampled_color = color_sum / samples;

            *pixel = Rgb(sampled_color.sqrt().to_u8_range().into());
        });

    let end = Instant::now();

    buffer.save("final-render.png").expect("Error saving image");
    let duration = end - start;
    println!("Rendered in {} ms", duration.as_millis());
}

fn random_scene() -> HitTarget {
    let mut world = HitTarget::new();
    let ground = Arc::new(Lambertian::new(Color::white() / 2.));
    (*world).push(Arc::new(Sphere::new(vec3!(0, -1000, 0), 1000., ground)));

    let glass = Arc::new(Dielectric::new(1.5));
    let brown_clay = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let silver = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));

    for a in -11..11 {
        for b in -11..11 {
            let a: f64 = a.into();
            let b: f64 = b.into();
            let random_number = Random::f64();
            let center = vec3!(a + 0.9 * random_number, 0.2, b + 0.9 * random_number);

            if (center - vec3!(4, 0.2, 0)).magnitude() > 0.9 {
                let material: Arc<dyn Material> = match random_number {
                    x if x < 0.8 => {
                        let albedo = Color::random_inclusive_between(0., 1.)
                            * Color::random_inclusive_between(0., 1.);
                        Arc::new(Lambertian::new(albedo))
                    }
                    x if x < 0.95 => {
                        let albedp = Color::random_inclusive_between(0.5, 1.);
                        let fuzz_factor = Random::f64_inclusive_between(0., 0.5);
                        Arc::new(Metal::new(albedp, fuzz_factor))
                    }
                    _ => glass.clone(),
                };
                (*world).push(Arc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    (*world).push(Arc::new(Sphere::new(vec3!(0, 1, 0), 1., glass)));
    (*world).push(Arc::new(Sphere::new(vec3!(-4, 1, 0), 1., brown_clay)));
    (*world).push(Arc::new(Sphere::new(vec3!(4, 1, 0), 1., silver)));
    world
}
