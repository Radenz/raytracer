fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let red = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let green = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let blue = 0.25;

            let red_byte = (255.999 * red) as u8;
            let green_byte = (255.999 * green) as u8;
            let blue_byte = (255.999 * blue) as u8;

            println!("{} {} {}", red_byte, green_byte, blue_byte);
        }
    }
}
