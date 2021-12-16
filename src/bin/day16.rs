use std::fs;

enum Packet {
    Literal(u64, u64),
    Operator(u64, u64, Vec<Packet>),
}

impl Packet {
    fn evaluate(&self) -> u64 {
        match &self {
            Self::Literal(_, v) => *v,
            Self::Operator(_, op, children) => {
                let child_values: Vec<u64> = children.iter().map(Packet::evaluate).collect();
                match *op {
                    0 => child_values.iter().sum(),
                    1 => child_values.iter().product(),
                    2 => *child_values.iter().min().unwrap(),
                    3 => *child_values.iter().max().unwrap(),
                    5 => (child_values[0] > child_values[1]) as u64,
                    6 => (child_values[0] < child_values[1]) as u64,
                    7 => (child_values[0] == child_values[1]) as u64,
                    _ => panic!("{}", op),
                }
            }
        }
    }

    fn version_sum(&self) -> u64 {
        match &self {
            &Packet::Literal(v, _) => *v,
            &Packet::Operator(v, _, subs) => v + subs.iter().map(Self::version_sum).sum::<u64>(),
        }
    }
}

fn base10(bits: &[char]) -> u64 {
    u64::from_str_radix(&String::from_iter(bits.iter()), 2).unwrap()
}

fn parse_literal(bits: &[char]) -> (u64, usize) {
    let mut binary_num = String::new();
    for (i, cs) in bits.chunks(5).enumerate() {
        binary_num.push_str(&String::from_iter(cs[1..5].iter()) as &str);
        if cs[0] == '0' {
            return (u64::from_str_radix(&binary_num, 2).unwrap(), (i + 1) * 5);
        }
    }
    panic!("{:?}", bits);
}

fn parse_packet(binary: &[char]) -> (Packet, usize) {
    let mut i = 0;
    let version = base10(&binary[i..i + 3]);
    i += 3;
    let type_id = base10(&binary[i..i + 3]);
    i += 3;
    if type_id == 4 {
        let (literal, consumed) = parse_literal(&binary[i..]);
        return (Packet::Literal(version, literal), consumed + i);
    }
    let length_type_id = base10(&binary[i..i + 1]);
    i += 1;
    let mut subpackets = Vec::new();
    if length_type_id == 0 {
        let packet_end = base10(&binary[i..i + 15]) as usize + i + 15;
        i += 15;
        while i < packet_end {
            let (subpack, consumed) = parse_packet(&binary[i..]);
            i += consumed;
            subpackets.push(subpack);
        }
    } else {
        let num_subpackets = base10(&binary[i..i + 11]) as usize;
        i += 11;
        for _ in 0..num_subpackets {
            let (subpack, consumed) = parse_packet(&binary[i..]);
            i += consumed;
            subpackets.push(subpack);
        }
    }
    (Packet::Operator(version, type_id, subpackets), i)
}

fn main() {
    let hex = fs::read_to_string("inputs/day16.txt").unwrap();
    let mut binary = Vec::with_capacity(hex.len() * 4);
    for i in 0..hex.len() {
        let bin = format!("{:04b}", u64::from_str_radix(&hex[i..i + 1], 16).unwrap());
        binary.extend(bin.chars());
    }
    let top_packet = parse_packet(&binary).0;
    println!("Part 1: {}", top_packet.version_sum());
    println!("Part 2: {}", top_packet.evaluate());
}
