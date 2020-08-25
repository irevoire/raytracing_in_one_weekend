use rand::Rng;
use raytracing_in_one_weekend::*;

fn main() {
    // image properties
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let mut world = World::new();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2));

    let ground = Sphere::new(Point3::new(0, -100.5, -1), 100., &material_ground);
    let center = Sphere::new(Point3::new(0, 0, -1), 0.5, &material_center);
    let left = Sphere::new(Point3::new(-1, 0, -1), 0.5, &material_left);
    let right = Sphere::new(Point3::new(1, 0, -1), 0.5, &material_right);

    world.push(&ground);
    world.push(&center);
    world.push(&left);
    world.push(&right);

    // camera
    let camera = Camera::default();

    // ppm formatting
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Color::default();

            for _ in 0..=samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                color += r.color(&world, max_depth);
            }
            println!("{}", color.color(samples_per_pixel));
        }
    }
}
