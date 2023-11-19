
#[derive(PartialEq, Eq, Debug)]
pub struct DnsClass(pub u16);
impl DnsClass {

    pub fn new(val: u16) -> DnsClass {
        DnsClass(val)
    }

}


pub mod DnsClasses {
    use super::DnsClass;


    pub const IN: DnsClass = DnsClass(1);
    // pub const UNKNOWN: DnsClass = Dn
}