use trace_math::ray::Ray;
use trace_math::vec3::{Point, Vec3};
use trace_math::utils::math_utils;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = math_utils::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalized();
        let u = Vec3::cross(vup, w).normalized();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w * focus_dist;
        let lens_radius = aperture / 2.;

        Camera { origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::rand_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset
        )
    }

    pub fn get_u(&self) -> Vec3 {
        self.u
    }

    pub fn get_v(&self) -> Vec3 {
        self.v
    } 

    pub fn get_w(&self) -> Vec3 {
        self.w
    } 
}