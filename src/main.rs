use std::fs::File;
use std::io::prelude::*;

struct Image {
    width: u32,
    height: u32,
}

fn main() -> std::io::Result<()> {
    let img_width = 256;
    let img_height = 256;
    //let img = Image { width: img_width, img_height: 256 };

    let mut file = File::create("img.ppm")?;
    file.write_all(b"P3\n256 256\n255\n")?;

    for j in (0..img_width).rev() {
        for i in 0..img_height {
            let r = (i as f64) / (img_width - 1) as f64;
            let g = (j as f64) / (img_height - 1) as f64;
            let b = 0.25;

            let ir = (255.9999 * r) as u32;
            let ig = (255.9999 * g) as u32;
            let ib = (255.9999 * b) as u32;

            let rgb = format!("{} {} {}\n", ir, ig, ib);
            //println!("{} {} {}", ir, ig, ib);
            file.write_all(rgb.as_bytes())?; 
        }
    }

    Ok(())
}
