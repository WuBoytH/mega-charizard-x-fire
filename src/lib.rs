#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

pub static mut MEGAZARDX_SLOT : [bool; 8] = [false; 8];

mod lizardon;
mod api;

#[skyline::main(name = "megazard_x_fire")]
pub fn main() {
    api::install();
    lizardon::install();
}