
//extern crate raster;
extern crate image;

// use image::GenericImage;
// use image::{ImageBuffer, Rgb, Pixel};

use geom::Tri2d;

pub fn draw_tri_2d(buf: &mut image::ImageBuffer<image::Luma<u8>, Vec<u8>>, tri: &Tri2d) {
    // println!("Drawing first line!");    // DEBUG
    draw_line(buf, tri.p1, tri.p2);
    // println!("Drawing second line!");   // DEBUG
    draw_line(buf, tri.p2, tri.p3);
    // println!("Drawing third line!");    // DEBUG
    draw_line(buf, tri.p1, tri.p3);
}

pub fn draw_line(buf: &mut image::ImageBuffer<image::Luma<u8>, Vec<u8>>, 
                  p1: (f64, f64), p2: (f64, f64)) {
    // Make sure points don't go out of bounds
    assert!(p1.0 as u32 <= buf.width() && p1.1 as u32 <= buf.height() );
    assert!(p2.0 as u32 <= buf.width() && p2.1 as u32 <= buf.height() );

    // Sort out vertices by which has the lower x value
    let (x1, x2, y1, y2) = if p1.0 > p2.0 { (p2.0, p1.0, p2.1, p1.1) } 
                           else           { (p1.0, p2.0, p1.1, p2.1) };

    // Derive the ranges between vertices, for each axis
    let (dx, dy) = ( (p2.0 - p1.0), (p2.1 - p1.1) );

    // Draw vertical line, then exit function
    if dx == 0.0 {
        // println!("({} {})-({} {}): \tVertical line", x1, y1, x2, y2);   // DEBUG
        for y in if y1 > y2 { (y2 as u32)..(y1 as u32) } else { (y1 as u32)..(y2 as u32) } {
            buf.put_pixel(x1 as u32, y as u32, image::Luma([255u8]));
        }
        // Job is done, exit
        return;
    }
    
    // Find slope of line, now that x/0 is guarenteed not to happen
    let slope = dy / dx;

    // Check if slope is positive
    if slope >= 0.0 { 
        if slope <= 1.0 {
            bresenham_oct1(buf, x1, y1, x2, slope); 
        } else {
            bresenham_oct2(buf, x1, y1, y2, slope.recip()); 
        }
    } else { // Then if slope is negative
        if slope >= -1.0 {
            bresenham_oct8(buf, x1, y1, x2, slope); 
        } else {
            bresenham_oct7(buf, y1, x2, y2, slope.recip()); 
        }
    }
} ////////////

// Assumming that the vertices are sorted so that x1 < x2
// Plots a line with 0 <= slope <= 1
fn bresenham_oct1(buf: &mut image::ImageBuffer<image::Luma<u8>, Vec<u8>>, 
                  x1: f64, y1: f64, x2: f64, slope: f64) {
    let mut err = 0.0;
    let mut y = y1 as u32;

    for x in (x1 as u32)..(x2 as u32) {
        buf.put_pixel(x as u32, y, image::Luma([255u8]));
        err += slope;
        if err >= 0.5 {
            y += 1u32;
            err -= 1f64;
        }
    }
    // println!("({} {})-({} {}) \tLine in first octant graphed.", x1, y1, x2, y2); // DEBUG
}

// Assumming that the vertices are sorted so that x1 < x2
// Plots lines with slope greater than 1
// This function must be given the inverse of slope
fn bresenham_oct2(buf: &mut image::ImageBuffer<image::Luma<u8>, Vec<u8>>, 
                  x1: f64, y1: f64, y2: f64, inv_slope: f64) {
    let mut err = 0.0;
    let mut x = x1 as u32;

    for y in (y1 as u32)..(y2 as u32) {
        buf.put_pixel(x as u32, y, image::Luma([255u8]));
        err += inv_slope;
        if err >= 0.5 {
            x += 1u32;
            err -= 1f64;
        }
    }
    // println!("({} {})-({} {}) \tLine in second octant graphed.", x1, y1, x2, y2); // DEBUG
}

// Assumming that the vertices are sorted so that x1 < x2
// Plots lines with -1 <= slope < 0
fn bresenham_oct8(buf: &mut image::ImageBuffer<image::Luma<u8>, Vec<u8>>, 
                  x1: f64, y1: f64, x2: f64, slope: f64) {
    let mut err = 0.0;
    let mut y = y1 as u32;
    // println!("{}", y);          // MUTHERFUGGIN DEBUG

    for x in (x1 as u32)..(x2 as u32) {
        // println!("x:{}\t y:{}\t err:{}\t slope:{}", x, y, err, slope);  // DEBUG
        buf.put_pixel(x as u32, y, image::Luma([255u8]));
        err += slope;
        if err <= -0.5 && y != 0 {
            y -= 1u32;
            err += 1f64;
        }
    }
    // println!("({} {})-({} {}) \tLine in eigthth octant graphed.", x1, y1, x2, y2); // DEBUG
}

// Assumming that the vertices are sorted so that x1 < x2
// Plots lines with slope < -1
// This function must be given the inverse of slope
fn bresenham_oct7(buf: &mut image::ImageBuffer<image::Luma<u8>, Vec<u8>>, 
                  y1: f64, x2: f64, y2: f64, inv_slope: f64) {
    let mut err = 0.0;
    let mut x = x2 as u32;

    for y in (y2 as u32)..(y1 as u32) {
        // println!("x:{}\t y:{}\t err:{}\t slope:{}", x, y, err, inv_slope);   // DEBUG
        buf.put_pixel(x as u32, y, image::Luma([255u8]));
        err += inv_slope;
        if err <= -0.5 && x != 0 {
            x -= 1u32;
            err += 1f64;
        }
    }
    // println!("({} {})-({} {}) \tLine in seventh octant graphed.", x1, y1, x2, y2); // DEBUG
}

// Barycentric attempt
pub fn fill_tri2d(buf: &mut image::ImageBuffer<image::Luma<u8>, Vec<u8>>, tri: &Tri2d) {
    let x_min = tri.p1.0.min(tri.p2.0.min(tri.p3.0)) as u32;
    let x_max = tri.p1.0.max(tri.p2.0.max(tri.p3.0)) as u32;
    let y_min = tri.p1.1.min(tri.p2.1.min(tri.p3.1)) as u32;
    let y_max = tri.p1.1.max(tri.p2.1.max(tri.p3.1)) as u32;
    // println!("X: min {}\tmax {}", x_min, x_max);
    // println!("Y: min {}\tmax {}", y_min, y_max);
    for x in x_min..x_max {
        for y in y_min..y_max {
            let divisor = (tri.p2.1 - tri.p3.1) * (tri.p1.0 - tri.p3.0) + (tri.p3.0 - tri.p2.0) * (tri.p1.1 - tri.p3.1);

            let alpha = ((tri.p2.1 - tri.p3.1) * (x as f64 - tri.p3.0) + (tri.p3.0 - tri.p2.0) * (y as f64 - tri.p3.1)) / divisor;
            let beta  = ((tri.p3.1 - tri.p1.1) * (x as f64 - tri.p3.0) + (tri.p1.0 - tri.p3.0) * (y as f64 - tri.p3.1)) / divisor;
            let gamma = 1.0f64 - alpha - beta;
            // println!("Alpha: {}\t Beta: {}\t Gamma: {}", alpha, beta, gamma);   // DEBUG
            if alpha >= 0f64 && beta  >= 0f64 && gamma >= 0f64 {
                if (x+y) % 5 == 0 { // Fancy fill effect
                    buf.put_pixel(x,y, image::Luma([200u8]));
                } else {
                    buf.put_pixel(x,y, image::Luma([  0u8]));
                }
            }   // if
        }   // for y
    }   // for x
}   // close function

pub fn fill_tri2d_rgb(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, tri: &Tri2d) {
    let mut x_min = tri.p1.0.min(tri.p2.0.min(tri.p3.0)) as u32;
    let mut x_max = tri.p1.0.max(tri.p2.0.max(tri.p3.0)) as u32;
    let mut y_min = tri.p1.1.min(tri.p2.1.min(tri.p3.1)) as u32;
    let mut y_max = tri.p1.1.max(tri.p2.1.max(tri.p3.1)) as u32;

    // Clip within boundaries of buffer
    if tri.p1.0.min(tri.p2.0.min(tri.p3.0)) < 0.0 { x_min = 0 };    // Can't check if x_min is
    if x_max > buf.width() { x_max = buf.width() };                 // is below 0 because its a
    if tri.p1.1.min(tri.p2.1.min(tri.p3.1)) < 0.0 { y_min = 0 };    // u32, so you have to recheck
    if y_max > buf.height() { y_max = buf.height() };               // the f64 values from the Tri2d

    // println!("X: min {}\tmax {}", x_min, x_max);
    // println!("Y: min {}\tmax {}", y_min, y_max);

    // Precompute the divisor just to save a little computing time
    let divisor = ((tri.p2.1 - tri.p3.1) * (tri.p1.0 - tri.p3.0) + (tri.p3.0 - tri.p2.0) * (tri.p1.1 - tri.p3.1)).recip();

    for x in x_min..x_max {
        for y in y_min..y_max {

            let alpha = ((tri.p2.1 - tri.p3.1) * (x as f64 - tri.p3.0) + (tri.p3.0 - tri.p2.0) * (y as f64 - tri.p3.1)) * divisor;
            let beta  = ((tri.p3.1 - tri.p1.1) * (x as f64 - tri.p3.0) + (tri.p1.0 - tri.p3.0) * (y as f64 - tri.p3.1)) * divisor;
            let gamma = 1.0f64 - alpha - beta;
            // println!("Alpha: {}\t Beta: {}\t Gamma: {}", alpha, beta, gamma);   // DEBUG
            if alpha >= 0f64 && beta  >= 0f64 && gamma >= 0f64 {
                buf.put_pixel(x,y, image::Rgb([(255f64 * alpha) as u8, (255f64 * beta) as u8, (255f64 * gamma) as u8]));
            }   // if
        }   // for y
    }   // for x
}   // close function
