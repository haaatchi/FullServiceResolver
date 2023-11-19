
#[derive(PartialEq, Eq, Debug)]
pub struct DnsType(pub u16);
impl DnsType {

    pub fn new(val: u16) -> DnsType {
        DnsType(val)
    }

}


pub mod DnsTypes {
    use super::DnsType;


    pub const A: DnsType = DnsType(1);
    pub const CNAME: DnsType = DnsType(5);
    // pub const UNKNOWN: DnsType = Dn
}