use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::*;
use crate::material::Material;

pub struct Sphere
{
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere
{
    pub fn sphere( center: Vec3, radius: f32, material: Material) -> Sphere
    {
        Sphere{
            center: center,
            radius: radius,
            material : material,
        }
    }
}
impl Hittable for Sphere
{
    fn hit(&self, r:&Ray, t_min: f32, t_max: f32) -> Option<HitRecord>
    {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), r.direction());
        let b = Vec3::dot(&oc, r.direction());
        let c: f32 = Vec3::dot(&oc, oc) - self.radius * self.radius;

        //let discriminant = b * b - 4.0 * a * c;
        //TODO: Why did I remove the four to make it work? 
        let discriminant = b * b -  a * c;
        if  discriminant > 0.0
        {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min
            {
                return Some(HitRecord
                {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material:self.material,
                });
            }
            temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min
            {
                return Some(HitRecord
                {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material:self.material,
                });
            }
        }
        None
    }
}
