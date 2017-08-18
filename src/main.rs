extern crate nix;
extern crate libc;
extern crate pnetlink;

use std::io;
use std::net::Ipv4Addr;
use std::str::FromStr;

use nix::fcntl;
use nix::sys::stat::Mode;
use pnetlink::packet::netlink::NetlinkConnection;
use pnetlink::packet::route::link::{Links};
use pnetlink::packet::route::addr::{IpAddr, Scope, Addresses};

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

    let mut conn = NetlinkConnection::new();
    let link = conn.get_link_by_name("tun0").unwrap().unwrap();
    println!("{}", link.get_index());

    conn.add_addr(&link, IpAddr::V4(Ipv4Addr::from_str("10.0.0.2").unwrap()),
                  Scope::Link);
    conn.link_set_up(link.get_index()).unwrap();

    let mut ln = String::new();
    io::stdin().read_line(&mut ln);
}
