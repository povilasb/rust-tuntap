use libc::IF_NAMESIZE;
use libc;

use std::process::Command;

use nix::fcntl;
use nix::sys::stat::Mode;

pub struct Device {
    name: String,
    addresses: String,
}

impl Device {
    pub fn new(name: String, addresses: String) -> Device {
        Device {
            name: name,
            addresses: addresses,
        }
    }

    pub fn up(&self) {
        let fid = fcntl::open("/dev/net/tun", fcntl::O_RDWR, Mode::empty()).unwrap();

        let dev_name = &self.name[..];
        create_vnet_device(fid, dev_name);

        Command::new("ip")
            .args(&["addr", "add", "10.0.0.0/24", "dev", dev_name])
            .spawn()
            .expect("Failed to assign IP address.");

        Command::new("ip")
            .args(&["link", "set", "dev", dev_name, "up"])
            .spawn()
            .expect("Failed to set interface up");
    }
}

const TUNSETIFF : u64 = 0x400454ca;

fn create_vnet_device(tun_file: i32, name: &str) -> Result<i32, i32> {
    let ifreq = InterfaceRequest::with_name(name);
    unsafe {
        match libc::ioctl(tun_file, TUNSETIFF, &ifreq) {
            -1 => Err(-1),
            _ => Ok(0),
        }
    }
}

pub enum VirtualDeviceType {
    Tun = 1,
    Tap = 2,
}

#[repr(C)]
pub struct InterfaceRequest {
    name: [u8; IF_NAMESIZE],
    flags: u16,
}

impl InterfaceRequest {
    pub fn with_name(name: &str) -> InterfaceRequest {
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
