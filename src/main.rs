extern crate nix;
extern crate libc;

use std::io;

use nix::fcntl;
use nix::sys::stat::Mode;
use libc::{ioctl, IF_NAMESIZE};

const TUNSETIFF : u64 = 0x400454ca;

enum VirtualDeviceType {
    Tun = 1,
    Tap = 2,
}

#[repr(C)]
struct InterfaceRequest {
    name: [u8; IF_NAMESIZE],
    flags: u16,
}

impl InterfaceRequest {
    fn with_name(name: &str) -> InterfaceRequest {
        let mut ifreq = InterfaceRequest {
            name: [0; IF_NAMESIZE],
            flags: VirtualDeviceType::Tun as u16
        };
        for (i, c) in name.as_bytes().iter().take(IF_NAMESIZE).enumerate() {
            ifreq.name[i] = *c;
        }
        ifreq
    }
}

fn main() {
    let fid = fcntl::open("/dev/net/tun", fcntl::O_RDWR, Mode::empty()).unwrap();
    let ifreq = InterfaceRequest::with_name("tun0");
    unsafe {
        let ret = ioctl(fid, TUNSETIFF, &ifreq);
        println!("{}", ret);
    }

    let mut ln = String::new();
    io::stdin().read_line(&mut ln);
}
