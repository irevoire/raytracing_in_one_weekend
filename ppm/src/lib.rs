use raytracing_in_one_weekend::Color;

pub fn output() {
    let width = 256;
    let height = 256;
    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for j in 0..height {
        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.25;

            let color = Color::new(r, g, b);

            println!("{}", color.color());
        }
    }
}
