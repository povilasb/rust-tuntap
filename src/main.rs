extern crate nix;
extern crate libc;

use std::io;

mod tuntap;

fn main() {
    let tuntap = tuntap::Device::new("tun0".to_string(), "10.0.1.0/24".to_string());
    tuntap.up();

    let mut ln = String::new();
    io::stdin().read_line(&mut ln);
}
