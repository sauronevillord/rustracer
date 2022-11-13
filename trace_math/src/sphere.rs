use crate::hittable::Hittable;
use crate::hit_info::HitInfo;
use crate::vec3::{Vec3, Point};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Box<dyn Material + 'static>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Box<dyn Material + 'static>) -> Sphere {
        Sphere{ center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0. {
            None
        } else {
            let sqrt_disc = discriminant.sqrt();
            let root = ( -half_b - sqrt_disc ) / a;
            if (root < t_min) || (t_max < root) {
                None
            } else {
                let t = root;
                let point = r.at(t);
                let out_normal = ( point - self.center ) / self.radius;
                let material = &*self.material;
                Some(HitInfo::new(point, out_normal, t, material, r))
            }
        }
    }
}