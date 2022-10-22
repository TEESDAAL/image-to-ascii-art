use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let path = if args.len() == 1 {
        "walnut.jpg"
    } else {
        &args[1][..]
    };
    let img = ImageReader::open(path).unwrap().decode().unwrap();

    print!("{}", get_image(img, 4));
}

fn get_image(img: DynamicImage, scale: u32) -> String {
    let mut ascii_art = String::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            if y % (scale * 2) != 0 || x % scale != 0 {
                continue;
            }
            let pixel = img.get_pixel(x, y);
            let strength = ((pixel[0] as f32 / 3. + pixel[1] as f32 / 3. + pixel[2] as f32 / 3.)
                as f32
                * (pixel[3] as f32 / 255.))
                .round() as u8;
            ascii_art.push(get_str_ascii(strength));
        }
        if y % (scale * 2) == 0 {
            ascii_art.push('\n');
        }
    }
    ascii_art
}

fn get_str_ascii(strength: u8) -> char {
    let characters = [' ', '.', '`', ',', '-', '~', '+', 'o', 'w', 'm', '%', '@'];
    let index = ((characters.len() - 1) as f32 * strength as f32 / 255.).round() as usize;

    return characters[index as usize];
}
