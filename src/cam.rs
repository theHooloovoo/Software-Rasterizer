
pub struct Camera {
    pub loc:    [f64; 3],
    pub xy_rot: f64,
    pub z_rot:  f64,

    pub fov: [f64; 2],
    pub width:  f64,
    pub height: f64,

    pub min_render_dist: f64,
    pub max_render_dist: f64,
}

impl Camera {
    pub fn new(h_fov: f64, v_fov: f64) -> Camera {
        Camera {
            loc: [0.0, 0.0, 0.0],
            xy_rot: 0.0,
            z_rot:  0.0,
            fov: [h_fov, v_fov],
            width:  h_fov.tan(),
            height: v_fov.tan(),
            min_render_dist: 1.0,
            max_render_dist: 1000.0,
        }
    }

}
