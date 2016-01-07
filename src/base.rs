extern crate libc;

use libc::c_int;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn get_country(mousex: c_int, mousey: c_int) -> CString {
    let mousepoint = Point::new(mousex as isize, mousey as isize);
    for country in PARSED_COUNTRIES.iter() {
        for hitbox in country.hitboxes.iter() {
            if hitbox.point_within(&mousepoint) {
                return CString::new(country.name.clone()).unwrap();
            }

        }
    }
    CString::new("No Country Found").unwrap()
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }
}

struct Point {
    pub x: isize,
    pub y: isize,
}

struct Hitbox {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

impl Hitbox {
    fn point_within(&self, point: &Point) -> bool {
        (point.x >= self.xmin) && (point.x <= self.xmax) && (point.y >= self.ymin) &&
        (point.y <= self.ymax)
    }
}

struct Country<'a> {
    pub name: &'a str,
    pub hitboxes: &'a [Hitbox],
}
