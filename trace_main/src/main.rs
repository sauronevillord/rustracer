use std::env;
use simple_bar::ProgressBar;

use trace_utils;
use trace_math::vec3::{Point, RGBColor, Vec3};
use trace_math::hittable::HittableList;
use trace_math::utils::rand_utils;
use trace_camera::camera::Camera;

use trace_math::sphere::Sphere;
use trace_math::material;

const RATIO_WIDTH: f64 = 3.;
const RATIO_HEIGHT: f64 = 2.;
const DEFAULT_HEIGHT: u32 = 200;
const SAMPLES_PER_PIXEL: u32 = 20;
const MAX_DEPTH: i32 = 50;

fn main() {
    // Image Preparations
    let args: Vec<String> = env::args().collect();
    
    let aspect_ratio = if args.len() < 3 {
        RATIO_WIDTH / RATIO_HEIGHT
    } else {
        args[1].parse::<f64>().unwrap() / args[2].parse::<f64>().unwrap()
    };

    let height = DEFAULT_HEIGHT;
    let width = (height as f64 * aspect_ratio) as u32;

    let mut out_img = trace_utils::initialize_ppm(width, height);

    // Scene
    //let scene = HittableList::rand_scene();
    let mut scene = HittableList::empty();
    
    let mat_ground = Box::new(material::Lambertian::new(RGBColor::new(0.8, 0.8, 0.)));
    let mat_right = Box::new(material::Metal::new(RGBColor::new(0.7, 0.7, 0.4), 0.0));
    let mat_left = Box::new(material::Dielectric::new(1.5));
    let mat_center = Box::new(material::Lambertian::new(RGBColor::new(0.1, 0.1, 0.8)));

    let sphere_ground = Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100., mat_ground));
    let sphere_center = Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, mat_center));
    let sphere_left = Box::new(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, mat_left));
    let sphere_right = Box::new(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, mat_right));

    scene.add(sphere_ground);
    scene.add(sphere_center);
    scene.add(sphere_left);
    scene.add(sphere_right);

    // Camera

    let lookfrom = Point::new(0.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus
    );

    // Progress Bar
    let mut bar = ProgressBar::cargo_style(height, 80);

    // Rendering
    for i in (0..height).rev() {
        bar.next();
        render_row(width, height, i, &scene, &camera, &mut out_img);
    }
    println!();

    trace_utils::save_ppm(out_img).unwrap();
}

fn render_row(
    width: u32,
    height: u32,
    i: u32,
    scene: &HittableList,
    camera: &Camera,
    out_img: &mut String
    )
{
    for j in 0..width {
        let mut pixel_color = RGBColor::zero();
        for _ in 0..SAMPLES_PER_PIXEL {
            let (u, v) = (
                (rand_utils::rand_f64() + j as f64) / (width as f64 - 1.),
                (rand_utils::rand_f64() + i as f64) / (height as f64 - 1.)
            );
            let r = camera.get_ray(u, v);
            pixel_color += r.ray_color(&scene, MAX_DEPTH);
        }
        trace_utils::add_pixel_to_ppm(out_img, pixel_color, SAMPLES_PER_PIXEL);
    }
}
