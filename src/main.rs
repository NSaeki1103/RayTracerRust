mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn hit_sphere
(
    center : Vec3,
    radius : f32,
    r : &Ray
) -> bool
{
    let oc = r.origin() - center;
    let a = Vec3::dot(&r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(&oc, r.direction());
    let c = Vec3::dot(&oc, oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0

}
fn color(r: &Ray) -> Vec3
{
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r)
    {
        return Vec3::new(1.0, 0.0, 0.0)
    }
    if hit_sphere(Vec3::new(0.5, 0.2, -0.9), 0.5, r)
    {
        return Vec3::new(0.0, 1.0, 0.0)
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t : f32 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
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


    println!("P3\n{} {}\n{}", width, height, MAXVALUE);

    for j in (0..height).rev()
    {
        for i in 0..width
        {
            let u: f32 = i as f32 / width as f32;
            let v: f32 = j as f32 / height as f32;
            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col : Vec3 = color(&r);
            let b: f32 = 0.2;

            let ir: i32 =(255.99 * col.r()) as i32;
            let ig: i32 =(255.99 * col.g()) as i32;
            let ib: i32 =(255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

}