#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod lizardon;

#[skyline::main(name = "megazard_x_fire")]
pub fn main() {
    lizardon::install();
}