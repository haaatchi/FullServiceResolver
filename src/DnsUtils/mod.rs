use std::{cell::RefCell, rc::Rc};

use crate::dns_type::{DnsType, DnsTypes};
// use crate::dns_type::DnsType::DnsTypes;
pub mod Utils;
// Header
#[derive(Debug)]
pub struct HeaderSection<'a> {
    length: usize,
    buf: &'a [u8],
}
impl HeaderSection<'_> {
    fn new(buffer: &[u8]) -> HeaderSection {
        HeaderSection {
            length: 12,
            buf: &buffer[0..12],
        }
    }

    pub fn question_count(&self) -> u16 {
        ((self.buf[4] as u16) << 8) + (self.buf[5] as u16)
    }

    pub fn answer_count(&self) -> u16 {
        ((self.buf[6] as u16) << 8) + (self.buf[7] as u16)
    }

    pub fn authority_count(&self) -> u16 {
        ((self.buf[8] as u16) << 8) + (self.buf[9] as u16)
    }

    pub fn additional_count(&self) -> u16 {
        ((self.buf[10] as u16) << 8) + (self.buf[11] as u16)
    }

    pub fn get_query_or_response_flag(&self) -> u8 {
        // return 0 : query
        // return 1 : response

        self.buf[2] >> 7
    }

    // pub fn set_response_flag(&mut self) {
    //     self.buf[2] = 1;
    // }
}

// Question
#[derive(Debug)]
struct Question {
    domain_name: String,
    domain_type: u16,
    domain_class: u16,
}
impl Question {
    fn new(buffer: &[u8]) -> (Question, usize) {
        let (len, domain_name) = Utils::ascii_to_string(&buffer);
        let domain_type = (buffer[len] as u16) << 8 + (buffer[len + 1] as u16);
        let domain_class = (buffer[len + 2] as u16) << 8 + (buffer[len + 3] as u16);
        (
            Question {
                domain_name,
                domain_type,
                domain_class,
            },
            len + 4,
        )
    }
}
#[derive(Debug)]
pub struct QuestionSection<'a> {
    length: usize,
    buf: &'a [u8],
    questions: Vec<Question>,
    length_of_question: Vec<usize>,
}
impl QuestionSection<'_> {
    fn new(buffer: &[u8], nums: u16) -> QuestionSection {
        let mut questions: Vec<Question> = Vec::<Question>::new();
        let mut index = 0;
        let mut length_of_question: Vec<usize> = Vec::<_>::new();
        for i in 0..nums {
            let (question, len) = Question::new(&buffer[index..]);
            questions.push(question);
            index = index + len;
            length_of_question.push(len);
        }

        QuestionSection {
            length: index,
            buf: &buffer,
            // nums,
            questions,
            length_of_question,
        }
    }

    pub fn domain_name(&self) -> Vec<String> {
        let mut queries: Vec<String> = Vec::<String>::new();

        for q in &self.questions {
            queries.push(q.domain_name.clone());
        }

        queries
    }
}

// ResourceRecord
#[derive(Debug)]
pub struct ResourceRecord<'a> {
    domain_name: String,
    domain_type: DnsType,
    domain_class: u16,
    ttl: u32,
    rd_length: u16,
    rdata: &'a [u8],
}
// trait ResourceRecord {}
impl ResourceRecord<'_> {
    fn new(buffer: &[u8]) -> (ResourceRecord, usize) {
        let mut len = 0;
        let mut domain_name = String::new();
        if (buffer[0] == 192) && (buffer[1] == 12) {
            len = 2;
        } else {
            (len, domain_name) = Utils::ascii_to_string(&buffer);
        }
        let domain_type = DnsType(((buffer[len] as u16) << 8) + (buffer[len + 1] as u16));
        let domain_class = ((buffer[len + 2] as u16) << 8) + (buffer[len + 3] as u16);
        let ttl = ((buffer[len + 4] as u32) << 24)
            + ((buffer[len + 5] as u32) << 16)
            + ((buffer[len + 6] as u32) << 8)
            + (buffer[len + 7] as u32);
        let rd_length = ((buffer[len + 8] as u16) << 8) + (buffer[len + 9] as u16);
        let rdata = &buffer[len + 10..len + 10 + (rd_length as usize)];
        (
            ResourceRecord {
                domain_name,
                domain_type,
                domain_class,
                ttl,
                rd_length,
                rdata,
            },
            len + 10 + rd_length as usize,
        )
    }
    fn rdata_to_string(&self) -> String {
        match self.domain_type {
            DnsTypes::A => {
                // rdata is ipv4
                let ipv4 = self.rdata[0].to_string()
                    + "."
                    + &self.rdata[1].to_string()
                    + "."
                    + &self.rdata[2].to_string()
                    + "."
                    + &self.rdata[3].to_string();
                ipv4
            }
            DnsTypes::CNAME => {
                let (len, cname) = Utils::ascii_to_string(self.rdata);
                cname
            }
            _ => {
                return "UNKNOWN".to_string();
            }
        }
    }
    pub fn print_answer(&self, query: String) -> String {
        let class_str = match self.domain_type {
            DnsTypes::A => "A",
            DnsTypes::CNAME => "CNAME",
            _ => {
                println!("domain type : {:?}", self.domain_type);
                "UNKNOW"
            }
        };
        let record = self.rdata_to_string();
        println!("{query}\t{}\t_IN_\t{}\t{}", self.ttl, class_str, record);
        // println!("{:#?}", self);

        match self.domain_type {
            DnsTypes::A => query,
            DnsTypes::CNAME => record,
            _ => {
                println!("domain type : {:?}", self.domain_type);
                "UNKNOW".to_string()
            }
        }
    }
}

// Answer
pub type Answer<'a> = ResourceRecord<'a>;
#[derive(Debug)]
pub struct AnswerSection<'a> {
    length: usize,
    buf: &'a [u8],

    nums: u16,
    answers: Vec<Answer<'a>>,
    length_of_answer: Vec<usize>,
}
impl AnswerSection<'_> {
    fn new(buffer: &[u8], nums: u16) -> AnswerSection {
        let mut answers: Vec<Answer> = Vec::<Answer>::new();
        let mut index = 0;
        let mut length_of_answer: Vec<usize> = Vec::<_>::new();
        for i in 0..nums {
            let (answer, len) = Answer::new(&buffer[index..]);
            answers.push(answer);
            index = index + len;
            length_of_answer.push(len);
        }

        AnswerSection {
            length: index,
            buf: &buffer,
            nums,
            answers,
            length_of_answer,
        }
    }
    pub fn print_answers(&self, queries: Vec<String>) {
        let mut query = queries[0].clone();
        for a in &self.answers {
            query = a.print_answer(query);
        }
    }
}

// Authority
type Authority<'a> = ResourceRecord<'a>;
#[derive(Debug)]
struct AuthoritySection<'a> {
    length: usize,
    buf: &'a [u8],

    nums: u16,
    authoritys: Vec<Authority<'a>>,
    length_of_authority: Vec<usize>,
}
impl AuthoritySection<'_> {
    fn new(buffer: &[u8], nums: u16) -> AuthoritySection {
        let mut authoritys: Vec<Authority> = Vec::<Authority>::new();
        let mut index = 0;
        let mut length_of_authority: Vec<usize> = Vec::<_>::new();
        for i in 0..nums {
            let (authority, len) = Authority::new(&buffer[index..]);
            authoritys.push(authority);
            index = index + len;
            length_of_authority.push(len);
        }

        AuthoritySection {
            length: index,
            buf: &buffer,
            nums,
            authoritys,
            length_of_authority,
        }
    }
}

// AdditionalSection
type Additional<'a> = ResourceRecord<'a>;
#[derive(Debug)]
struct AdditionalSection<'a> {
    length: usize,
    buf: &'a [u8],

    nums: u16,
    additionals: Vec<Additional<'a>>,
    length_of_additional: Vec<usize>,
}
impl AdditionalSection<'_> {
    fn new(buffer: &[u8], nums: u16) -> AdditionalSection {
        let mut additionals: Vec<Additional> = Vec::<Additional>::new();
        let mut index = 0;
        let mut length_of_additional: Vec<usize> = Vec::<_>::new();
        for i in 0..nums {
            let (additional, len) = Additional::new(&buffer[index..]);
            additionals.push(additional);
            index = index + len;
            length_of_additional.push(len);
        }

        AdditionalSection {
            length: index,
            buf: &buffer,
            nums,
            additionals,
            length_of_additional,
        }
    }
}

// DNS packet.
#[derive(Debug)]
pub(crate) struct DnsPacket<'a> {
    // _buffer: Rc<RefCell<&'a [u8]>>,
    buffer: &'a [u8],
    pub header: HeaderSection<'a>,
    pub question: QuestionSection<'a>,
    pub answer: AnswerSection<'a>,
    authority: AuthoritySection<'a>,
    additional: AdditionalSection<'a>,
}

impl DnsPacket<'_> {
    pub(crate) fn new(buffer: &[u8]) -> DnsPacket {
        let mut total_length = 0;

        let _buffer2 = Rc::new(RefCell::new(buffer));
        let header = HeaderSection::new(&buffer);
        total_length = 12;

        // let mut tl = &total_length;
        let question = QuestionSection::new(&buffer[total_length..], header.question_count());
        // println!("{:#?}", question);
        total_length = total_length + question.length;

        let answer = AnswerSection::new(&buffer[total_length..], header.answer_count());
        // println!("{:#?}", answer);
        total_length = total_length + answer.length;

        let authority = AuthoritySection::new(&buffer[total_length..], header.authority_count());
        // println!("{:#?}", authority);
        total_length = total_length + authority.length;

        let additional = AdditionalSection::new(&buffer[total_length..], header.additional_count());
        // println!("{:#?}", additional);
        total_length = total_length + additional.length;

        DnsPacket {
            buffer,
            header,
            question,
            answer,
            authority,
            additional,
        }
    }
}
