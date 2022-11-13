use crate::hit_info::HitInfo;
use crate::ray::Ray;
use crate::material;
use crate::utils::rand_utils::{rand_f64, rand_f64_range};
use crate::vec3::{RGBColor, Point};
use crate::sphere::Sphere;

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}

type Shape = Box<dyn Hittable>;

pub struct HittableList(Vec<Shape>);

impl HittableList {
    pub fn empty() -> HittableList {
        HittableList(vec![])
    }

    pub fn rand_scene() -> HittableList {
        let mut scene = HittableList::empty();

        let ground_material = Box::new(material::Lambertian::new(RGBColor::new(0.5, 0.5, 0.5)));
        let ground_sphere = Box::new(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material));
        scene.add(ground_sphere);

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rand_f64();
                let center = Point::new(a as f64 + 0.9 * rand_f64(), 0.2, b as f64 + 0.9*rand_f64());

                if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo = RGBColor::rand() * RGBColor::rand();
                        let sphere_material = Box::new(material::Lambertian::new(albedo));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                        scene.add(sphere);
                    } else if choose_mat < 0.95 {
                        let albedo = RGBColor::rand_range(0.5, 1.0);
                        let fuzz = rand_f64_range(0.0, 0.5);
                        let sphere_material = Box::new(material::Metal::new(albedo, fuzz));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                        scene.add(sphere);
                    } else {
                        let sphere_material = Box::new(material::Dielectric::new(1.5));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                        scene.add(sphere);
                    }
                }
            }
        }

        let mat1 = Box::new(material::Dielectric::new(1.5));
        let sphere1 = Box::new(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1));
        scene.add(sphere1);

        let mat2 = Box::new(material::Lambertian::new(RGBColor::new(0.4, 0.2, 0.1)));
        let sphere2 = Box::new(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2));
        scene.add(sphere2);

        let mat3 = Box::new(material::Metal::new(RGBColor::new(0.7, 0.6, 0.5), 0.0));
        let sphere3 = Box::new(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3));
        scene.add(sphere3);

        scene
    }

    pub fn from<T>(objs: T) -> HittableList 
        where
            T: IntoIterator<Item = Shape>
    {
        let mut objects: Vec<Shape> = vec![];
        for obj in objs.into_iter() {
            objects.push(obj);
        }

        HittableList(objects)
    }

    pub fn add(&mut self, obj: Shape) {
        self.0.push(obj);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let mut closest_so_far = t_max;
        let mut hit_rec = None;

        for shape in self.0.iter() {
            let hit_r = shape.hit(r, t_min, closest_so_far);

            hit_rec = match hit_r {
                Some(hit_record) => {
                    if !hit_record.get_front_face(){
                        hit_record.print();
                    }
                    closest_so_far = hit_record.get_t();
                    Some(hit_record)
                },
                None => continue
            };
        }

        hit_rec
    }
}