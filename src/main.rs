mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittablelist;
mod camera;
mod material;

use camera::Camera;
use hittablelist::HittableList;
use vec3::Vec3;
use sphere::Sphere;
use ray::Ray;
use hittable::Hittable;
use material::{scatter, Material};

use rand::prelude::*;


fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 
{
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) 
    {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attentuation = Vec3::default();


        if depth < 50 && scatter(&rec.material, r, &rec, &mut attentuation, &mut scattered) 
        {
            return attentuation * color(&scattered, world, depth + 1);
        } 
        else 
        {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } 
    else 
    {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn main()
{
    let width : i32 = 400;
    let height : i32 = 400;
    let samples= 100;
    let MAXVALUE : i32 = 255;

    let mut list : Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::sphere
    (
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Material::Lambertian 
        {
            albedo: Vec3::new(0.8, 0.3, 0.3),
       },
    )));
    list.push(Box::new(Sphere::sphere
    (
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        },
    )));
    list.push(Box::new(Sphere::sphere
    (
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 0.3,
        },
    )));
    list.push(Box::new(Sphere::sphere
    (
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.8, 0.8, 0.8),
            fuzz: 0.3,
        },
    )));

    let world = HittableList::new(list);

    let cam = Camera::camera();
    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n{}", width, height, MAXVALUE);

    for j in (0..height).rev()
    {
        for i in 0..width
        {
            let mut col = Vec3::default();

            for _ in 0..samples
            {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / height as f32;
                let r = &cam.get_ray(u, v);
                col = col + color(&r, &world,0);
            }

            col = col / samples as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir: i32 =(255.99 * col.r()) as i32;
            let ig: i32 =(255.99 * col.g()) as i32;
            let ib: i32 =(255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

}