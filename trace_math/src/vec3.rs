use std::ops;

use crate::utils::rand_utils;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type RGBColor = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x +
        self.y * self.y +
        self.z * self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x +
        u.y * v.y +
        u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0., y: 0., z: 0. }
    }

    pub fn ones() -> Vec3 {
        Vec3 { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn rand() -> Vec3 {
        Vec3 { x: rand_utils::rand_f64(), y: rand_utils::rand_f64(), z: rand_utils::rand_f64() }
    }

    pub fn rand_range(min: f64, max: f64) ->Vec3 {
        let x = rand_utils::rand_f64_range(min, max);
        let y = rand_utils::rand_f64_range(min, max);
        let z = rand_utils::rand_f64_range(min, max);

        Vec3 { x, y, z }
    }

    pub fn rand_in_unit_sphere() -> Vec3 {
        let max_length = rand_utils::rand_f64();
        
        let x_perc = rand_utils::rand_f64_range(0., 1.);
        let y_perc = rand_utils::rand_f64_range(0., 1. - x_perc);
        let z_perc = 1. - (x_perc + y_perc);

        let x_sign = match rand_utils::rand_bool() {
            true => 1.,
            false => -1.
        };

        let y_sign = match rand_utils::rand_bool() {
            true => 1.,
            false => -1.
        };

        let z_sign = match rand_utils::rand_bool() {
            true => 1.,
            false => -1.
        };
        
        let x = (max_length * x_perc).sqrt() * x_sign;
        let y = (max_length * y_perc).sqrt() * y_sign;
        let z = (max_length * z_perc).sqrt() * z_sign;

        Vec3 { x, y, z }
    }

    pub fn rand_in_unit_disk() -> Vec3 {
        let max_length = rand_utils::rand_f64();

        let x_perc = rand_utils::rand_f64_range(0., 1.);
        let y_perc = 1. - x_perc;

        let x_sign = match rand_utils::rand_bool() {
            true => 1.,
            false => -1.
        };

        let y_sign = match rand_utils::rand_bool() {
            true => 1.,
            false => -1.
        };

        let x = (max_length * x_perc).sqrt() * x_sign;
        let y = (max_length * y_perc).sqrt() * y_sign;
        let z = 0.0;

        Vec3 { x, y, z }
    }

    pub fn rand_unit_vector() -> Vec3 {
        Vec3::rand_in_unit_sphere().normalized()
    }

    pub fn rand_in_emisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::rand_in_unit_sphere();
        match Vec3::dot(in_unit_sphere, normal) > 0. {
            true => in_unit_sphere,
            false => -in_unit_sphere,
        }
    }

    pub fn is_near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn reflect(&self, v: Vec3) -> Vec3 {
        *self - v * (2. * Vec3::dot(*self, v))
    }

    pub fn refract(&self, n: Vec3, eta_i_over_eta_t: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(-*self, n), 1.0);
        let r_perp = (*self + (n * cos_theta)) * eta_i_over_eta_t;
        let r_parallel = n * -(f64::sqrt(f64::abs(1.0 - r_perp.length_squared())));
        r_perp + r_parallel
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;      
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;      
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;      
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;      
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
