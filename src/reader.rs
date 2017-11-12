
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use geom::Tri3d;

pub fn load_obj(vert_list: &mut Vec<[f64; 3]>, face_list: &mut Vec<Tri3d>, f_in: &str) -> Result<(u32, u32),()> {
    // Open file and panic if not found
    // (Find a nicer way to do this)
    let mut file = match File::open(f_in) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {} {}", f_in, why.description() ),
    };
    
    // Pull contents of file into a single string
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    // Convert contents into a vec of &str, splitting at each line
    let mut data = contents.lines().collect::<Vec<&str>>();
    let mut data2: Vec<Vec<&str>> = Vec::with_capacity(1000);

    for n in data {
        data2.push( n.split_whitespace().collect::<Vec<&str>>() );
    }

    for n in data2 {
        // Check if line is vertex data
        if n[0] == "v" && n.len() >= 4 { vert_list.push([ n[1].parse::<f64>().unwrap(),
                                                          n[2].parse::<f64>().unwrap(),
                                                          n[3].parse::<f64>().unwrap() ] ) };
    }


    Ok((1, 1))
}

pub fn parse_vertex(line: &Vec<&str>, vert_list: &mut Vec<[f64;3]>) {
}


