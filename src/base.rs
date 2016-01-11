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
    // board hitbox
    // make this math block run once somehow?
    let boardbox = Hitbox {
        xmin: 0,
        xmax: BOARDW,
        ymin: 0,
        ymax: BOARDH,
    };
    if !boardbox.point_within(&mousepoint) {
        // mouse not in board at all
        return CString::new("null").unwrap();
    }
    drop(boardbox);
    for country in PARSED_COUNTRIES.iter() {
        for hitbox in country.hitboxes.iter() {
            if hitbox.point_within(&mousepoint) {
                return CString::new(country.name.clone()).unwrap();
            }
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

struct Hitbox {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Hitbox {
    fn point_within(&self, point: &Point) -> bool {
        (point.x >= self.xmin) && (point.x <= self.xmax) &&
        (point.y >= self.ymin) && (point.y <= self.ymax)
    }
}

struct Country<'a> {
    pub name: &'a str,
    pub hitboxes: &'a [Hitbox],
}
