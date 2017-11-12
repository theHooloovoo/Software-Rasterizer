
/*
 * TODO:
 *   Add getopts
 *   Add support for obj files
 *   Add code for camera translation
 *   maybe do texture stuff
*/

//extern crate raster;
extern crate image;

// use raster::Image;

// use image::GenericImage;
// use image::{ImageBuffer, Rgb, Pixel};

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

mod cam;
mod geom;
mod drawer;
mod reader;
mod screen;

fn parse_file_test(f_in: &str) {
    let f = File::open(f_in);

    let f = match f {
        Ok(file) => file,
        Err(_)   => File::create("".to_string() ).unwrap(),
    };
}

// Assuming the file exists
fn parse_tri2d_list(f_in: &mut File, t_list: &mut Vec<geom::Tri2d>) {
    let mut s = String::new();
    // Read contents in
    match f_in.read_to_string(&mut s) {
        Err(_) => println!("File couldn't be opened"),
        Ok(_)  => println!("File opened Successfully!"),
    }
    // Split content into chunks
    let words = s.split_whitespace();
    let mut num_list: Vec<f64> = Vec::new();
    for w in words {
        num_list.push(w.parse::<f64>().unwrap() );
    }
    assert!(num_list.len() % 6 == 0);

    for n in 0..num_list.len() {
        if n % 6 == 0 {
            t_list.push(geom::Tri2d::new((num_list[n  ], num_list[n+1]),
                                         (num_list[n+2], num_list[n+3]),
                                         (num_list[n+4], num_list[n+5]) ) );
        }
    }
    println!("Parsed the file of triangles");
}


fn main() {
    let mut verts:  Vec<[f64; 3]> = Vec::new();
    let mut tri3ds: Vec<geom::Tri3d> = Vec::new();
    let (mut v, mut f) = (0u32,0u32);
    let load_result = reader::load_obj(&mut verts, &mut tri3ds, "rsrc/cube.obj");
    match load_result {
        Err(_) => println!("Couldn't open file"),
        Ok( result )  => { v = result.0;
                           f = result.1 },
    };
    /////////////////

    let img_x = 1000u32;
    let img_y = 1000u32;

    // Seems like a good starting size
    let mut triangles: Vec<geom::Tri2d> = Vec::with_capacity(1000);

    let f_in = Path::new("rsrc/tri_list_01");
    let display = f_in.display();
    let mut tri_file = match File::open(&f_in) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description() ),
        Ok(file)  => file,
    };


    parse_tri2d_list(&mut tri_file, &mut triangles);

    let mut imgbuf = image::ImageBuffer::new(img_x, img_y);
    let mut rgb_buf =image::ImageBuffer::new(img_x, img_y);


    let ref mut f_out = File::create (&Path::new("image.png")).unwrap();
    let _ = image::ImageLuma8(imgbuf).save(f_out, image::PNG);

    let ref mut f_out = File::create (&Path::new("rgb_image.png")).unwrap();
    let _ = image::ImageRgb8(rgb_buf).save(f_out, image::PNG);
}
