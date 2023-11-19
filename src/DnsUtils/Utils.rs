use ascii::AsciiStr;


pub(crate) fn ascii_to_string(dns_name: &[u8]) -> (usize, String) {
    // println!("in dns_name {:?}", dns_name);
    // [0] == 0 => 終端。
    // [0] = b11xxxxxx => skip
    // 0<[0]<b11xxxxxx => 読み込み.

    if dns_name[0] == 0 {
        return (1, "".to_string());
    }

    if (dns_name[0] >> 6) == 3 {
        // calc skips
        let bytes_of_skip =
            ((((dns_name[0] - (3 << 6)) as u16) << 8) + dns_name[1] as u16) as usize;
        if dns_name.len() > bytes_of_skip {
            let (_, name) = ascii_to_string(&dns_name[bytes_of_skip..]);
            // let (_, name) = ascii_to_string(&dns_name[2 + bytes_of_skip..]);
            return (2, name);
        } else {
            return (2, "".to_string());
        }
    }

    let len = dns_name[0] as usize;
    let (l, name) = ascii_to_string(&dns_name[(len + 1)..]);
    let label = unsafe { AsciiStr::from_ascii_unchecked(&dns_name[1..(len + 1)]) };
    (1 + len + l, label.to_string() + "." + &name)
}