use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    #[allow(non_snake_case)]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;

        let a = r.direction().dot(&r.direction());
        let b = 2.0 * oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);

        let D = b.powi(2) - 4.0 * a * c;
        if D > 0.0 {
            let sqrt_part = b.powi(2) - a * c;

            for temp in [(-b - sqrt_part) / a, (-b + sqrt_part) / a] {
                if temp < t_max && temp > t_min {
                    rec.t = temp;
                    rec.p = r.point_at_parameter(rec.t);
                    rec.normal = (rec.p - self.center) / self.radius;
                    return true;
                }
            }
        }

        false
    }
}
