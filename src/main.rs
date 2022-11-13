mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittablelist;

use hittablelist::HittableList;
use vec3::Vec3;
use sphere::Sphere;
use ray::Ray;
use hittable::{HitRecord, Hittable}; 

fn color(r: &Ray, world: &HittableList) -> Vec3
{
    let mut rec = HitRecord::default();  
    if world.hit(&r, 0.0, std::f32::MAX, &mut rec)
    {
        return 0.5 * Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0)
    }
    else
    {
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t : f32 = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }

}
fn main()
{
    let width : i32 = 200;
    let height : i32 = 100;
    let MAXVALUE : i32 = 255;

    let lower_left_corner : Vec3 = Vec3::new( -2.0, -1.0,-1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical : Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin : Vec3 = Vec3::new( 0.0, 0.0,0.0);

    let mut list : Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0),0.5)));
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0),100.0)));
    let world = HittableList::new(list);
    println!("P3\n{} {}\n{}", width, height, MAXVALUE);

    for j in (0..height).rev()
    {
        for i in 0..width
        {
            let u: f32 = i as f32 / width as f32;
            let v: f32 = j as f32 / height as f32;
            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col : Vec3 = color(&r, &world);
            let b: f32 = 0.2;

            let ir: i32 =(255.99 * col.r()) as i32;
            let ig: i32 =(255.99 * col.g()) as i32;
            let ib: i32 =(255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

}