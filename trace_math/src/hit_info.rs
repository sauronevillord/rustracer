use crate::vec3::{Vec3, Point};
use crate::ray::Ray;
use crate::material::Material;

pub struct HitInfo<'a> {
    point: Point,
    normal: Vec3,
    t: f64,
    material: &'a dyn Material,
    front_face: bool,
}

impl<'a> HitInfo<'a> {
    pub fn new(point: Point, out_normal: Vec3, t: f64, material: &'a dyn Material, r: Ray) -> HitInfo {
        let (front_face, normal) = HitInfo::set_normal(r, out_normal);
        HitInfo { point, normal, t, material, front_face }
    }

    pub fn get_point(&self) -> Point {
        self.point
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn get_front_face(&self) -> bool {
        self.front_face
    }

    pub fn get_material(&self) -> &'a dyn Material {
        self.material
    }

    fn set_normal(r: Ray, out_normal: Vec3) -> (bool, Vec3) {
        let front_face = Vec3::dot(r.direction, out_normal) < 0.;
        let normal = if front_face {
            out_normal
        } else {
            -out_normal
        };

        (front_face, normal)
    }

    pub fn print(&self) {
        println!("{{");
        println!("  point: {:?}", self.get_point());
        println!("  normal: {:?}", self.get_normal());
        println!("  t: {:?}", self.get_t());
        println!("  front_face: {:?}", self.get_front_face());
        println!("}}");
    }
}