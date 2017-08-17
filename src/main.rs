extern crate nix;
extern crate libc;

use std::io;

use nix::fcntl;
use nix::sys::stat::Mode;

mod tuntap;

const TUNSETIFF : u64 = 0x400454ca;

fn create_vnet_device(tun_file: i32, name: &str) -> Result<i32, i32> {
    let ifreq = tuntap::InterfaceRequest::with_name(name);
    unsafe {
        match libc::ioctl(tun_file, TUNSETIFF, &ifreq) {
            -1 => Err(-1),
            _ => Ok(0),
        }
    }
}

fn main() {
    let fid = fcntl::open("/dev/net/tun", fcntl::O_RDWR, Mode::empty()).unwrap();
    create_vnet_device(fid, "tun0");

    let mut ln = String::new();
    io::stdin().read_line(&mut ln);
}
