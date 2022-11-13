use crate::hittable::Hittable;
use crate::vec3::{Vec3, RGBColor};
use crate::hittable::HittableList;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self, scene: &HittableList, depth: i32) -> RGBColor {
        if depth <= 0 {
            RGBColor::zero()
        } else {
            match scene.hit(*self, 0.001, f64::INFINITY) {
                Some(hit) => {
                    match hit.get_material().scatter(*self, hit) {
                        Some((r,c)) => {
                            return c * r.ray_color(scene, depth-1);
                        },
                        None => {
                            return RGBColor::zero();
                        }
                    }
                },
                None => {
                    let unit_direction = self.direction.normalized();
                    let t = 0.5 * (unit_direction.y + 1.);
                    let sky_color = RGBColor::new(1., 1., 1.) * (1. - t) + RGBColor::new(0.5, 0.7, 1.) * t;
                    return sky_color;
                },
            }
        }
    }
}