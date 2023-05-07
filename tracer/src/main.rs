#![forbid(unsafe_code)]

use std::io::prelude::*;

const PPM_MAGIC: &'static str = "P3";

const IMAGE_WIDTH: u32 = u8::MAX as u32 + 1;
const IMAGE_HEIGHT: u32 = u8::MAX as u32 + 1;

fn main() -> std::io::Result<()> {
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    writeln!(
        out,
        "{}\t{} {}\t{}",
        PPM_MAGIC,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        u8::MAX
    )?;

    for row in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {} of {}", row, IMAGE_HEIGHT);

        for col in 0..IMAGE_WIDTH {
            let r = col as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = row as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25f32;

            fn i(x: f32) -> u8 {
                debug_assert!(x >= 0.0);
                debug_assert!(x <= 1.0);
                (255.999f32 * x) as u8
            }
            writeln!(out, "{} {} {}", i(r), i(g), i(b))?;
        }
    }

    out.flush()?;
    eprintln!("Done");
    Ok(())
}
