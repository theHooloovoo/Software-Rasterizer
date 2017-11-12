
pub struct Tri2d {
    pub p1: (f64, f64),
    pub p2: (f64, f64),
    pub p3: (f64, f64),
}

impl Tri2d {
    pub fn new(p_1: (f64, f64), p_2: (f64, f64), p_3: (f64, f64) ) -> Tri2d {
        Tri2d {
            p1: p_1,
            p2: p_2,
            p3: p_3,
        }
    }
}

pub struct Tri3d {
    pub data: [f64;9],
    pub single_sided: bool,
    pub wire_draw:    bool,
    pub render:       bool,
}
