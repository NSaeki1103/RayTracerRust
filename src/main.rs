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
    let mut rng = rand::thread_rng();
    let width : i32 = 1600;
    let height : i32 = 800;
    let samples= 100;
    let MAXVALUE : i32 = 255;

    let mut list : Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::sphere
    (
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian 
        {
            albedo: Vec3::new(0.5, 0.5, 0.5),
       },
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let centre = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 
                {
                    let albedo = Vec3::random() * Vec3::random();
                    list.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Lambertian { albedo },
                    )));
                }
                else if choose_mat < 0.95 
                {
                    let albedo = Vec3::random_init(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    list.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Metal { albedo, fuzz },
                    )));
                } 
                else 
                {
                    list.push(Box::new(Sphere::sphere(
                        centre,
                        0.2,
                        Material::Dielectric { ref_idx: 1.5 },
                    )));
                }
            }
        }
    }
    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric { ref_idx: 1.5 },
    )));

    let world = HittableList::new(list);

    let aspect_ratio = width as f32 / height as f32;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let apeture = 0.1;

    let cam = Camera::camera
    (
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        apeture,
        dist_to_focus,
    );

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