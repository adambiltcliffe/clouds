use noise::{NoiseFn, Perlin};

const W: u32 = 128;
const H: u32 = 128;
const TILES_W: u32 = 1;
const TILES_H: u32 = 1;

const LOWER_THRESHOLD: f64 = 0.2;
const HIGHER_THRESHOLD: f64 = 0.6;

fn gen(n: &impl NoiseFn<f64, 2>, x: f64, y: f64) -> f64 {
    n.get([x * 3., y * 6.]) * 1.0
        + n.get([x * 6., y * 15.]) * 0.8
        + n.get([x * 8., y * 20.]) * 0.6
        + n.get([x * 16., y * 40.]) * 0.4
}

fn tile(n: &impl NoiseFn<f64, 2>, tx: f64, ty: f64) -> f64 {
    gen(&n, tx, ty) * tx * ty
        + gen(&n, tx + 1., ty) * (1. - tx) * ty
        + gen(&n, tx, ty + 1.) * tx * (1. - ty)
        + gen(&n, tx + 1., ty + 1.) * (1. - tx) * (1. - ty)
}

fn main() {
    let mut buf = image::ImageBuffer::new(W * TILES_W, H * TILES_H);

    let billow = Perlin::new(781);

    for y in 0..H {
        for x in 0..W {
            let tx = x as f64 / W as f64;
            let ty = y as f64 / H as f64;
            let n = tile(&billow, tx, ty);
            let n2 = tile(&billow, tx, ty + 1. / H as f64);
            let c = if n > HIGHER_THRESHOLD || (n > LOWER_THRESHOLD && n2 + 0.1 < n) {
                image::Rgb([124, 77, 255u8])
            } else if n > LOWER_THRESHOLD {
                image::Rgb([69, 21, 234])
            } else {
                image::Rgb([49, 27, 146])
            };
            buf.put_pixel(x, y, c);
        }
    }

    for y in 0..H * TILES_H {
        for x in 0..W * TILES_W {
            if x >= W || y >= H {
                let pixel = *buf.get_pixel(x % W, y % H);
                buf.put_pixel(x, y, pixel);
            }
        }
    }

    buf.save("output.png").expect("could not save output");
}
