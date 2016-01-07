use std::path::Path;
use std::fs::copy;
use std::env;
use std::io::{File, Read};

const RKT_FNAME: &'static str = "rust_functions.rkt";
const COUNTRY_DATA: &'static str = "countrydata.txt";
const LIB_BEGIN: &'static str = "

";

macro_rules! generate_countries {
    ($filepath:expr) => {{
        let country_file = File::open($filepath).unwrap();
        let country_data = 
        let mut countries = Vec::new();
        let mut current_country = Country::new("");
        for line in data.split("\n") {
            if line != "" {
                let vars = line.split(" ")
                               .map(|s| s.to_string())
                               .collect::<Vec<String>>();
                if vars[0].parse::<isize>().is_err() {
                    // country name, add previous country to countries
                    countries.push(current_country);
                    current_country = Country::new(line);
                } else {
                    // hitbox for current country
                    let ireps = vars.iter()
                                    .map(|s| s.parse::<isize>().unwrap())
                                    .collect::<Vec<isize>>();
                    let hb = Hitbox::new(ireps[0], ireps[1], ireps[2], ireps[3]);
                    current_country.add_hitbox(hb);
                }
            }
        }
        countries.push(current_country);
        countries.remove(0);
        countries
    }}
}

fn main() {
    let out_var = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_var);
    let target_dir = out_dir.join("..").join("..").join("..");
    let manifest_var = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_var);
    let src_dir = manifest_dir.join("src");
    let rkt_src_path = src_dir.join(RKT_FNAME);
    let rkt_out_path = target_dir.join(RKT_FNAME);
    copy(rkt_src_path, rkt_out_path).expect(&format!("Failed to copy {}", RKT_FNAME));
    let country_data_path = src_dir.join(COUNTRY_DATA);
    let main_lib_path = src_dir.join("lib.rs");

}

struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Debug)]
struct Hitbox {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

impl Hitbox {
    fn new(xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> Hitbox {
        Hitbox {
            xmin: xmin,
            xmax: xmax,
            ymin: ymin,
            ymax: ymax,
        }
    }

    fn point_within(&self, point: &Point) -> bool {
        (point.x >= self.xmin) && (point.x <= self.xmax) && (point.y >= self.ymin) &&
        (point.y <= self.ymax)
    }
}

#[derive(Debug)]
struct Country {
    pub name: String,
    pub hitboxes: Vec<Hitbox>,
}

impl Country {
    fn new(name: &str) -> Country {
        Country {
            name: name.to_string(),
            hitboxes: Vec::new(),
        }
    }

    fn add_hitbox(&mut self, hb: Hitbox) {
        self.hitboxes.push(hb);
    }
}


