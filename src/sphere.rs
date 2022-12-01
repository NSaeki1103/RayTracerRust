use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::*;

pub struct Sphere
{
    center: Vec3,
    radius: f32,
}

impl Sphere
{
    pub fn sphere( center: Vec3, radius: f32) -> Sphere
    {
        Sphere{
            center: center,
            radius: radius,
        }
    }
}
impl Hittable for Sphere
{
    fn hit(&self, r:&Ray, t_min: f32, t_max: f32, rec: & mut HitRecord) -> bool
    {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), r.direction());
        let b = Vec3::dot(&oc, r.direction());
        let c = Vec3::dot(&oc, oc) - self.radius * self.radius;

        //let discriminant = b * b - 4.0 * a * c;
        //TODO: Why did I remove the four to make it work? 
        let discriminant = b * b -  a * c;
        if  discriminant > 0.0
        {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min
            {
                rec.set_t(temp);
                rec.set_p(r.point_at_parameter(rec.t()));
                rec.set_normal((rec.p() - self.center) / self.radius);
                return true;
            }
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min
            {
                rec.set_t(temp);
                rec.set_p(r.point_at_parameter(rec.t()));
                rec.set_normal((rec.p() - self.center) / self.radius);
                return true;
            }
        }
        return false;
    }
}
