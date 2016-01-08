use std::path::Path;
use std::fs::{File, copy};
use std::env;
use std::io::{Read, Write};

const BOARDW: isize = 1307;
const BOARDH: isize = 875;
const RKT_FNAME: &'static str = "rust_functions.rkt";
const COUNTRY_DATA: &'static str = "countrydata.txt";
const LIB_BASE: &'static str = "base.rs";

fn parse_countries(data: &str) -> Vec<Country> {
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
                // find out racket coord system
                let hb = Hitbox::new(BOARDW - ireps[0],
                                     BOARDW - ireps[0] + ireps[2],
                                     BOARDH - ireps[1],
                                     BOARDH - ireps[1] + ireps[3]);
                current_country.add_hitbox(hb);
            }
        }
    }
    countries.push(current_country);
    countries.remove(0);
    countries
}

fn generate_declaration(country: &Country) -> String {
    ["Country { \n    name: \"",
     &country.name as &str,
     "\",",
     "\n    hitboxes: &",
     &format!("{:#?}", &country.hitboxes) as &str,
     "\n}"]
        .join("")
}

fn generate_declarations(country_data: &[Country]) -> String {
    let mut countrydecs = "// countries".to_string();
    for country in country_data.iter() {
        countrydecs = [&countrydecs as &str, &generate_declaration(country) as &str].join(",\n");
    }
    ["static PARSED_COUNTRIES: &'static [Country<'static>] = &[", &countrydecs as &str, "];\n"]
        .join("\n")
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

    let lib_base_path = src_dir.join(LIB_BASE);
    let mut lib_base_file = File::open(lib_base_path)
                                .expect("Failed to open base lib file to read.");
    let mut lib_base = String::new();
    lib_base_file.read_to_string(&mut lib_base).expect("Failed to read base lib file.");

    let country_data_path = src_dir.join(COUNTRY_DATA);
    let mut country_data_file = File::open(country_data_path)
                                    .expect("Failed to open country data file to read.");
    let mut country_data_raw = String::new();
    country_data_file.read_to_string(&mut country_data_raw)
                     .expect("Failed to read raw country data.");
    drop(country_data_file);
    let country_data = parse_countries(&country_data_raw);

    let main_lib_path = src_dir.join("lib.rs");
    let mut main_lib_file = File::create(main_lib_path)
                                .expect("Failed to open main lib file to write.");
    main_lib_file.write_all([&lib_base as &str, &generate_declarations(&country_data) as &str]
                                .join("\n")
                                .as_bytes())
                 .expect("Failed to write genned code.");
    drop(main_lib_file);
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
