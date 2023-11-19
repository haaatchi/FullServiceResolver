use std::net::UdpSocket;

extern crate rand;

use rand::Rng;
pub mod dns_type;
// use dns_type::DnsTypes;

pub mod dns_class;

pub mod DnsUtils;

fn main() {
   
    println!("Hello, world!");
    let host_address = "192.168.1.241";
    let host_port = "10153";
    let socket = UdpSocket::bind(host_address.to_owned() + ":" + host_port).unwrap();

    loop {
        let mut buffer = [0u8; 1500];
        let (amount, source) = socket.recv_from(&mut buffer).unwrap();

        let mut buffer = &buffer[0..amount];

        let mut dns_packet = DnsUtils::DnsPacket::new(&mut buffer);

        println!("\n### query infomation.");
        // println!("{:#?}", dns_packet);
        println!("question_count\t : {}", dns_packet.header.question_count());
        println!("answer_count\t : {}", dns_packet.header.answer_count());
        println!("authority_count\t : {}", dns_packet.header.authority_count());
        println!("additional_count\t: {}", dns_packet.header.additional_count());
        println!("{:?}", dns_packet.question.domain_name());

        let host_address = "192.168.1.241";
        let mut rng = rand::thread_rng();
        let host_port = (rng.gen::<u32>() % 65536).to_string();

        let dest_address = "192.168.1.1";
        let dest_port = "53";
        let resolvesocket = UdpSocket::bind(host_address.to_owned() + ":" + &host_port);
        match resolvesocket {
            Ok(sock) => {

                match sock.send_to(buffer, dest_address.to_string() + ":" + dest_port){
                    Ok(v) => {
                        let mut _buffer = [0; 1500];
                        let (amount, source) = sock.recv_from(&mut _buffer).unwrap();
                        let _buffer = &_buffer[0..amount];
                        let dp_ = DnsUtils::DnsPacket::new(&_buffer);

                        println!("\n### answer infomation.");
                        // println!("{:#?}", dp_);
                        println!("question_count\t : {}", dp_.header.question_count());
                        println!("answer_count\t : {}", dp_.header.answer_count());
                        println!("authority_count\t : {}", dp_.header.authority_count());
                        println!("additional_count\t: {}", dp_.header.additional_count());
                        dp_.answer.print_answers(dns_packet.question.domain_name());
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {}
        }

        // dns_packet.header.set





    }
}
