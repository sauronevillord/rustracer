use crate::hit_info::HitInfo;
use crate::ray::Ray;
use crate::vec3::{RGBColor, Vec3};
use crate::utils::rand_utils;

type RayColor = (Ray, RGBColor);

pub trait Material {
    fn scatter(&self, ray: Ray,  hit_info: HitInfo) -> Option<RayColor>;
}

pub struct Lambertian {
    pub albedo: RGBColor,
}

impl Lambertian {
    pub fn new(albedo: RGBColor) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, hit_info: HitInfo) -> Option<RayColor> {
        let mut scatter_direction = hit_info.get_normal() + Vec3::rand_unit_vector();

        // Catch degenerate scatter direction
        // (when the direction is very near zero, which will lead to infinity and NaN problems)
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_info.get_normal();
        }

        let ray = Ray::new(hit_info.get_point(), scatter_direction);
        let attenuation = self.albedo;
        Some((ray, attenuation))
    }
}

pub struct Metal {
    pub albedo: RGBColor,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: RGBColor, fuzziness: f64) -> Metal {
        let fuzzy = fuzziness.clamp(0.0, 1.0);
        Metal { albedo, fuzziness: fuzzy }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_info: HitInfo) -> Option<RayColor> {
        let reflected = ray.direction.normalized().reflect(hit_info.get_normal());

        let scattered = Ray::new(
            hit_info.get_point(),
            reflected + Vec3::rand_in_unit_sphere() * self.fuzziness
        );
        let attenuation = self.albedo;

        if Vec3::dot(scattered.direction, hit_info.get_normal()) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)) * ((1. - ref_idx) / (1. + ref_idx));
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray,  hit_info: HitInfo) -> Option<RayColor> {
        let ref_ratio = match hit_info.get_front_face() {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index
        };

        let unit_direction = ray.direction.normalized();

        let cos_theta = Vec3::dot(-unit_direction, hit_info.get_normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ref_ratio * sin_theta > 1.0;
        let p = Dielectric::reflectance(cos_theta, ref_ratio) > rand_utils::rand_f64();
        let condition = cannot_refract || p;
        let direction = match condition {
            true => unit_direction.reflect(hit_info.get_normal()),
            false => unit_direction.refract(hit_info.get_normal(), ref_ratio),
        };

        let attenuation = RGBColor::ones();
        let scattered = Ray::new(hit_info.get_point(), direction);

        Some((scattered, attenuation))
    }
}