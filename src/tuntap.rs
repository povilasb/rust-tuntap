use libc::IF_NAMESIZE;

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
