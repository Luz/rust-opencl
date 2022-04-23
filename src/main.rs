extern crate ocl;
use ocl::ProQue;
extern crate image;
use std::error::Error;
extern crate colorous;
use colorous::Color;

fn step(width: u32, height: u32, invec: &Vec<f32>, outvec: &mut Vec<f32>) -> ocl::Result<()> {
    let src = include_str!("kernel.cl");
    let que = ProQue::builder().src(src).dims((width, height)).build()?;
    let input = que.create_buffer::<f32>()?;
    let output = que.create_buffer::<f32>()?;

    input.write(invec).enq()?;

    let kernel = que
        .kernel_builder("update")
        .arg(&input)
        .arg(&output)
        .arg(width)
        .arg(height)
        .build()?;
    unsafe {
        kernel.enq()?;
    }

    output.read(&mut *outvec).enq()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let width: u32 = 700;
    let height: u32 = 300;
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    // Create empty input buffer:
    let invec: Vec<f32> = vec![0.0; (width * height) as usize];

    // Optionally create some initial data:
    /*    let draw_height: usize = (height/10) as usize;
    let draw_offset: usize = draw_height*width as usize;
    let draw_width: usize = (width/10) as usize;
    for i in 0..draw_width {
        invec[i+draw_offset] = 1.0;
    }
    */

    let mut outvec: Vec<f32> = vec![0.0; (width * height) as usize];

    match step(width, height, &invec, &mut outvec) {
        Ok(()) => println!("Step executed"),
        Err(err) => eprintln!("Step failed: {}", err),
    };

    let gradient = colorous::TURBO;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (x, y) = (x as usize, y as usize);

        let scale = 0.33; // Max 0.5 if highest of data is 1
        let single_array_coord: usize = x + (width as usize) * y;
        let scaled_pixel: f64 = ((outvec[single_array_coord]) * scale) as f64;

        let Color { r, g, b } = gradient.eval_continuous(scaled_pixel + 0.5);
        *pixel = image::Rgb([r, g, b]);
    }
    imgbuf.save("output.png")?;

    Ok(())
}
