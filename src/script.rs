
#[skyline::from_offset(0x00e3bde0)]
fn get_int_arg(a1: i32) -> i32;

#[skyline::from_offset(0x00e3be90)]
fn get_float_arg(a1: i32) -> f32;

#[skyline::from_offset(0x00e3bf30)]
fn get_string_arg(a1: i32) -> *const u8;

pub struct Script;

impl Script {
    pub fn get_int_arg(index: i32) -> i32 {
        unsafe { get_int_arg(index) }
    }

    pub fn get_float_arg(index: i32) -> f32 {
        unsafe { get_float_arg(index) }
    }

    pub fn get_string_arg(index: i32) -> *const u8 {
        unsafe { get_string_arg(index) }
    }
}