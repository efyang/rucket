extern crate libc;

use libc::c_int;
use std::ffi::CString;

const BOARDW: i32 = 1307;
// 875 + 75 (75 for buttons) = 950
const BOARDH: i32 = 950;
static mut XPAD: i32 = 0;
static mut YPAD: i32 = 0;

#[no_mangle]
pub extern "C" fn get_countryliststr() -> CString {
    let countrystrings = PARSED_COUNTRIES.iter()
        .map(|c| c.name)
        .collect::<Vec<&str>>();
    let countrystring = countrystrings.join("|");
    CString::new(countrystring).unwrap()
}

#[no_mangle]
pub extern "C" fn set_padding(scrw: c_int, scrh: c_int) {
    unsafe {
        XPAD = (scrw - BOARDW)/2;
        YPAD = (scrh - BOARDH)/2;
    }
}

#[no_mangle]
pub extern "C" fn get_x_pad() -> c_int {
    unsafe {
        YPAD
    }
}

#[no_mangle]
pub extern "C" fn get_y_pad() -> c_int {
    unsafe {
        XPAD
    }
}

#[no_mangle]
pub extern "C" fn get_country(mousex: c_int, mousey: c_int) -> CString {
    let mousepadx;
    let mousepady;
    unsafe {
        mousepadx = mousex - XPAD;
        mousepady = mousey - YPAD
    }
    let mousepoint = Point::new(mousepadx, mousepady);
    if (mousepadx < 0) && (mousepadx > BOARDW) && (mousepady < 0) && (mousepadx > BOARDW) {
        // mouse not in board at all
        return CString::new("null").unwrap();
    }
    for country in PARSED_COUNTRIES.iter() {
        if country.point_within(&mousepoint) {
            return CString::new(country.name.clone()).unwrap();
        }
    }
    CString::new("null").unwrap()
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

struct Point {
    pub x: i32,
    pub y: i32,
}

struct Country<'a> {
    pub name: &'a str,
    pub points: &'a [Point],
}

impl<'a> Country<'a> {
    pub fn point_within(&self, pt: &Point) -> bool {
        let nvert = self.points.len();
        let x = pt.x;
        let y = pt.y;
        let mut j = nvert - 1;
        for i in 0..nvert {
            let ref ipt = self.points[i];
            let ref jpt = self.points[j];
            let xi = ipt.x;
            let yi = ipt.y;
            let xj = jpt.x;
            let yj = jpt.y;
            let intersect = ((yi > y) != (yj > y)) && (x < ((xj - xi) * (y - yi) / (yj - yi) + xi));
            if intersect {
                return intersect;
            }
            j = i;
        }
        false
    }

}
