extern crate image;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use cam::Camera;
use geom::Tri2d;
use geom::Tri3d;

pub struct Screen {
    pub width:  u32,
    pub height: u32,
    pub buf:    image::ImageBuffer<image::Rgb <u8>, Vec<u8>>,
    pub zbuf:   image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    pub cam:    Camera,

    pub color:  [u8; 3],
}

impl Screen {

    // x and y are pixel dimensions
    // h_fov and v_fov are fov of view for internal camera
    pub fn new(x: u32, y: u32, h_fov: f64, v_fov: f64) -> Screen {
        Screen {
            width: x,
            height: y,
            buf:  image::ImageBuffer::new(x,y),
            zbuf: image::ImageBuffer::new(x,y),
            cam: Camera::new(65f64.to_radians(), 45f64.to_radians()),
            color: [255u8,255u8,255u8,],
        }
    }

    pub fn project_point(&self, p: &[f64; 3]) -> (f64, f64) {
        // Given that the camera is at (0,0,0), and that the screen
        // lies at x=1, scale the given vector such that it lies on
        // the screen
        ( ((p[1] / p[0]) + 0.5 * self.cam.width) *  (self.width as f64  / self.cam.width),
          ((p[2] / p[0]) + 0.5 * self.cam.height) * (self.height as f64 / self.cam.height) )

    }

    pub fn clip_tri3d(&self, p: &Tri3d) {
    }



}
