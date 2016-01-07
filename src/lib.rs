#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

const COUNTRY_DATA: &'static str = include_str!("countrydata.txt");

struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }
}

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

    fn point_within(&self, point: Point) -> bool {
        (point.x >= self.xmin) && (point.x <= self.xmax) && (point.y >= self.ymin) &&
        (point.y <= self.ymax)
    }
}

struct Country {
    name: String,
    hitboxes: Vec<Hitbox>,
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

fn parse_countries(data: &str) -> Vec<Country> {
    let mut countries = Vec::new();
    let mut current_country = Country::new("");
    for line in data.split("\n") {
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
    countries.push(current_country);
    countries.remove(0);
    countries
}

// parse the countries into Vec<Country>

lazy_static! {
    static ref PARSED_COUNTRIES: Vec<Country> = parse_countries(&COUNTRY_DATA);
}

// provide list of all countries
// #[no_mangle]
// pub extern "C" fn
