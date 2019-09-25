extern crate virt;

use std::ptr;
use virt::connect::Connect;

fn main() {
    let conn = Connect::new(ptr::null_mut());

    match conn.get_uri() {
        Ok(u) => println!("Connected to hypervisor at '{}'", u),
        Err(e) => {
            panic!("Failed to get URI for hypervisor connection: code {}, message: {}",
                   e.code,
                   e.message);
        }
    };
}