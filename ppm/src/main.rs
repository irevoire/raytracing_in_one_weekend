use raytracing_in_one_weekend::*;

fn main() {
    // image properties
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // world
    let mut world = World::new();
    let tmp = Sphere::new(Point3::new(0, 0, -1), 0.5);
    world.push(&tmp);
    let tmp = Sphere::new(Point3::new(0, -100.5, -1), 100.);
    world.push(&tmp);

    // camera properties
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0, 0);
    let vertical = Vec3::new(0, viewport_height, 0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0, 0, focal_length);

    // ppm formatting
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = r.color(&world);

            println!("{}", color.color());
        }
    }
}
