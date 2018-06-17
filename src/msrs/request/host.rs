#[derive(Debug)]
pub struct HostDescription {
    ip: IpAddress,
    port: Port,
}

#[derive(Debug)]
pub enum IpAddress {
    Localhost,
    #[allow(dead_code)]
    IpV4(u8, u8, u8, u8),
    #[allow(dead_code)]
    IpV6, // useless for moment
}

pub type Port = u16;

impl Default for HostDescription {
    fn default() -> Self {
        HostDescription {
            ip: IpAddress::Localhost,
            port: 8080u16,
        }
    }
}

impl HostDescription {
    pub fn new(ip: IpAddress, port: Port) -> Self {
        HostDescription { ip, port }
    }
}