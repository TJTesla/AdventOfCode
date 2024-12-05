#![allow(dead_code)]

use std::fs::read_to_string;

fn hex_to_dec(character: char) -> String {
    match character {
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
        'A' | 'a' => "1010",
        'B' | 'b' => "1011",
        'C' | 'c' => "1100",
        'D' | 'd' => "1101",
        'E' | 'e' => "1110",
        'F' | 'f' => "1111",
        _ => panic!("The given character {} is not an accepted hexadecimal character", character)
    }.to_string()
}

fn parse_input(test: bool) -> String {
    let input = if test {
        // "8A004A801A8002F478".to_string()
        // "620080001611562C8802118E34".to_string()
        // "C0015000016115A2E0802F182340".to_string()
        // "A0016C880162017C3686B18A3D4780".to_string()
        // "EE00D40C823060".to_string()
        "38006F45291200".to_string()
    } else {
        read_to_string("input/year2021/day16.txt").unwrap()
    };

    let mut result = String::new();
    for char in input.chars() {
        result.push_str(&hex_to_dec(char));
    }

    return result;
}

enum Packet {
    LiteralValue {
        version: u8,
        type_id: u8,
        number: u128,
    },
    Operator {
        version: u8,
        type_id: u8,
        sub_packets: Vec<Packet>
    }
}

impl Packet {
    fn get_version(&self) -> u8 {
        use Packet::*;

        match *self {
            LiteralValue { version, type_id: _, number: _ } => version,
            Operator { version, type_id: _, sub_packets: _ } => version
        }
    }

    fn get_type_id(&self) -> u8 {
        use Packet::*;

        match *self {
            LiteralValue { version: _, type_id, number: _ } => type_id,
            Operator { version: _, type_id, sub_packets: _ } => type_id
        }
    }
}

fn collect_literal_value(input: &str, pointer: &mut usize) -> u128 {
    let mut last_byte = false;
    let mut result_string = String::new();
    while !last_byte {
        let byte = &input[*pointer..*pointer+5];
        if byte.starts_with("0") {
            last_byte = true;
        }
        result_string.push_str(&byte[1..5]);
        *pointer += 5;
    }

    return u128::from_str_radix(&result_string, 2).unwrap();
}

fn collect_sub_packets(input: &str, pointer: &mut usize, sub_packet_countdown: Option<u16>) -> Vec<Packet> {
    let mut packets = Vec::new();

    match sub_packet_countdown {
        None => {
            // Amount is given through bits -> Repeat reading packets, until no further bits to read
            while let Some(packet) = read(input, pointer) {
                packets.push(packet);
            }
        },
        Some(mut sub_packet_countdown) => {
            // Amount is given through number -> Count down while reading packets
            while sub_packet_countdown > 0 {
                packets.push(read(input, pointer).unwrap());
                sub_packet_countdown -= 1;
            }
            *pointer += 1;
        }
    }

    return packets;
}

fn read(input: &str, pointer: &mut usize) -> Option<Packet> {
    if *pointer > input.len()-6 {
        // Pointer is so much back that there can't be made another packet
        return None;
    }
    let version = u8::from_str_radix(&input[*pointer..*pointer+3], 2).unwrap();
    let type_id = u8::from_str_radix(&input[*pointer+3..*pointer+6], 2).unwrap();

    if type_id == 4 {
        *pointer += 6;
        return Some(Packet::LiteralValue {
            version,
            type_id,
            number: collect_literal_value(&input, pointer)
        });
    } else {
        fill_operator_packet(version, type_id, input, pointer)
    }
}

fn fill_operator_packet(version: u8, type_id: u8, input: &str, pointer: &mut usize) -> Option<Packet> {
    // Look how number of sub-packets is given
    let length_type = u8::from_str_radix(&input[*pointer+6..*pointer+7], 2).unwrap();
        
    let sub_packets;
    if length_type == 0 {
        // Next 15 bits is number of bits of the sub_packets
        let upper_bounds = if *pointer+22 >= input.len() {
            (*pointer + input.len() - *pointer) as u16
        } else {
            (*pointer+22) as u16
        };
        let mut bit_amount = u16::from_str_radix(&input[*pointer+7..upper_bounds as usize], 2).unwrap();
        *pointer += 22;
        let mut offset = 0;
        // For some reason a packet is allowed to say it is longer than the remaining string...
        if *pointer+(bit_amount as usize) >= input.len() {
            bit_amount = (input.len() - *pointer) as u16;
        }
        sub_packets = collect_sub_packets(
            &input[*pointer..*pointer+(bit_amount as usize)], 
            &mut offset,
            None
        );
        *pointer += offset;
    } else {
        // Next 11 bits is number of subpackets
        let packet_amount = u16::from_str_radix(&input[*pointer+7..*pointer+18], 2).unwrap();
        *pointer += 18;
        sub_packets = collect_sub_packets(
            input, 
            pointer, 
            Some(packet_amount)
        );
    }

    return Some(Packet::Operator {
        version,
        type_id,
        sub_packets
    });
}

fn add_versions(tree: &Packet) -> u128 {
    let mut current_sum = tree.get_version() as u128;

    if let Packet::Operator { version: _, type_id: _, sub_packets } = tree {
        for sub in sub_packets {
            current_sum += add_versions(sub);
        }
    }

    return current_sum;
}

pub fn part_one(test: bool) {
    let input = parse_input(test);

    let tree = read(&input, &mut 0).unwrap();
    println!("The sum is {}", add_versions(&tree));
}