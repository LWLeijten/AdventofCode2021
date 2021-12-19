use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Packet {
    version: i64,
    packet_type_id: i64,
    literal: Option<i64>,
    length_type_id: Option<i64>,
    subpackets: Option<Vec<Packet>>,
}

impl Packet {
    fn calc_version_score(self: Self) -> i64 {
        let mut score = self.version;
        match self.subpackets {
            Some(sp) => {
                for packet in sp {
                    score += packet.calc_version_score();
                }
            }
            None => {}
        }
        score
    }
}

fn read_input_from_file(path: &str) -> String {
    let mut input = String::new();
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    buf.lines()
        .for_each(|line: Result<String, std::io::Error>| {
            let line = line.unwrap();
            for c in line.chars() {
                input.push_str(hex_to_dec(&c));
            }
        });
    input
}

fn hex_to_dec(hex: &char) -> &str {
    match hex {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn bitstring_to_int(bitstring: &str) -> i64 {
    let mut result = 0;
    let base: i64 = 2;
    for (pos, bit) in bitstring.chars().into_iter().enumerate() {
        result += bit.to_digit(10).unwrap() as i64
            * base.pow((bitstring.len() - 1 - pos).try_into().unwrap());
    }
    result
}

fn parse_literal(input: &String, pos: &mut usize) -> i64 {
    let mut literal = String::new();
    loop {
        if input.chars().nth(*pos).unwrap() == '1' {
            let sub_bits = &input[*pos + 1..*pos + 5];
            *pos += 5;
            literal.push_str(sub_bits);
        } else {
            let sub_bits = &input[*pos + 1..*pos + 5];
            *pos += 5;
            literal.push_str(sub_bits);
            return bitstring_to_int(&literal);
        }
    }
}

fn parse_packet(input: &String, mut pos: &mut usize) -> Packet {
    let version = bitstring_to_int(&input[*pos..*pos + 3]);
    *pos += 3;
    let packet_type_id = bitstring_to_int(&input[*pos..*pos + 3]);
    *pos += 3;
    let mut literal = None;
    let mut length_type_id = None;
    let mut subpackets = None;
    match packet_type_id {
        4 => {
            literal = Some(parse_literal(&input, &mut pos));
        }
        _ => {
            length_type_id = Some(bitstring_to_int(&input[*pos..*pos + 1]));
            *pos += 1;
            if length_type_id.unwrap() == 0 {
                let length = bitstring_to_int(&input[*pos..*pos + 15]) as usize;
                *pos += 15;
                subpackets = Some(parse_packets(
                    input.clone()[*pos..*pos + length].to_string(),
                ));
                *pos += length;
            } else {
                let count = bitstring_to_int(&input[*pos..*pos + 11]);
                let mut inner_packets = vec![];
                *pos += 11;
                for _ in 0..count {
                    inner_packets.push(parse_packet(input, &mut pos));
                }
                subpackets = Some(inner_packets);
            }
        }
    }
    return Packet {
        version,
        packet_type_id,
        literal,
        length_type_id,
        subpackets,
    };
}

fn parse_packets(input: String) -> Vec<Packet> {
    let mut packets = vec![];
    let mut pos = 0;
    loop {
        if pos == input.len() || input[pos..].chars().all(|c| c == '0') {
            break;
        }
        let packet = parse_packet(&input, &mut pos);
        packets.push(packet);
    }
    packets
}

fn evaluate_packet(packet: &Packet) -> i64 {
    let subpackets = packet.subpackets.as_ref();
    match packet.packet_type_id {
        0 => {
            return subpackets.unwrap().iter().map(|p| evaluate_packet(p)).sum();
        }
        1 => {
            return subpackets
                .unwrap()
                .iter()
                .map(|p| evaluate_packet(p))
                .product();
        }
        2 => {
            return subpackets
                .unwrap()
                .iter()
                .map(|p| evaluate_packet(p))
                .min()
                .unwrap();
        }
        3 => {
            return subpackets
                .unwrap()
                .iter()
                .map(|p| evaluate_packet(p))
                .max()
                .unwrap();
        }
        4 => return packet.literal.unwrap(),
        5 => {
            return (evaluate_packet(&subpackets.unwrap()[0])
                > evaluate_packet(&subpackets.unwrap()[1])) as i64
        }
        6 => {
            return (evaluate_packet(&subpackets.unwrap()[0])
                < evaluate_packet(&subpackets.unwrap()[1])) as i64
        }
        7 => {
            return (evaluate_packet(&subpackets.unwrap()[0])
                == evaluate_packet(&subpackets.unwrap()[1])) as i64
        }
        _ => return 0,
    }
}

fn part1(packets: Vec<Packet>) -> i64 {
    let mut score = 0;
    for p in packets {
        score += p.calc_version_score();
    }
    score
}

fn part2(packets: Vec<Packet>) -> i64 {
    evaluate_packet(&packets[0])
}

fn main() {
    let input = read_input_from_file("input.txt");
    let packets = parse_packets(input);
    let part1 = part1(packets);
    println!("Solution to part 1: {}", &part1);
    let input2 = read_input_from_file("input.txt");
    let packets2 = parse_packets(input2);
    let part2 = part2(packets2);
    println!("Solution to part 2: {}", &part2);
}
