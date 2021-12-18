use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct HitableList {
    pub hitables: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn len(&self) -> usize {
        self.hitables.len()
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let temp_rec: HitRecord;
        let hit_anything = false;
        let closest_so_far = t_max;

        self.hitables
            .iter()
            .filter(|h| h.hit(r, t_min, closest_so_far, &mut temp_rec))
            .map(|h| {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = &mut temp_rec
            });
        hit_anything
    }
}
