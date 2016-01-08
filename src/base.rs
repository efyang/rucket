extern crate libc;
extern crate simd;

use libc::c_int;
use std::ffi::CString;
use simd::i32x4;

const BOARDW: i32 = 1307;
const BOARDH: i32 = 875;

#[no_mangle]
pub extern "C" fn get_x_pad(scrw: c_int) -> c_int {
    (scrw - BOARDW)/2
}

#[no_mangle]
pub extern "C" fn get_y_pad(scrh: c_int) -> c_int {
    (scrh - BOARDH)/2
}

#[no_mangle]
pub extern "C" fn get_country(mousex: c_int, mousey: c_int, xpad: c_int, ypad: c_int) -> CString {
    let mousepoint = Point::new(mousex, mousey);
    // board hitbox
    // make this math block run once somehow?
    let boardbox = Hitbox {
        xmin: 0,
        xmax: BOARDW,
        ymin: 0,
        ymax: BOARDH,
    };
    if !boardbox.point_within(&mousepoint, xpad, ypad) {
        // mouse not in board at all
        return CString::new("null").unwrap();
    }
    drop(xpad);
    drop(ypad);
    drop(boardbox);
    for country in PARSED_COUNTRIES.iter() {
        for hitbox in country.hitboxes.iter() {
            if hitbox.point_within(&mousepoint, xpad, ypad) {
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
    fn point_within(&self, point: &Point, xpad: i32, ypad: i32) -> bool {
        // simd for padding
        let hitboxvec = i32x4::new(self.xmin,
                                   self.xmax,
                                   self.ymin,
                                   self.ymax);
        let padvec = i32x4::new(xpad, xpad, ypad, ypad);
        let paddedvec = hitboxvec + padvec;
        drop(hitboxvec);
        drop(padvec);
        (point.x >= paddedvec.extract(0)) && (point.x <= paddedvec.extract(1)) &&
        (point.y >= paddedvec.extract(2)) && (point.y <= paddedvec.extract(3))
    }
}

struct Country<'a> {
    pub name: &'a str,
    pub hitboxes: &'a [Hitbox],
}
