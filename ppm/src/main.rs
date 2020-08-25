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
    let tmp = Sphere::new(Point3::new(0, 0, -1), 0.5);
    world.push(&tmp);
    let tmp = Sphere::new(Point3::new(0, -100.5, -1), 100.);
    world.push(&tmp);

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
