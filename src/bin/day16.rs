use aoc2021::read_strs;

/// This task has some quirks that require some special attention.
/// The input is a string of bits. It could represent a single literal value,
/// or a nested structure of subpackets.
/// It gets interesting when you arrive at the subpacket part.
/// The data that contains the subpacket is simply (again) a string of bits.
/// But there are two ways this can be interpreted:
/// - it has n subpackets, and stop reading after the n-th subpacket
/// - it has subpackets over the length of n bits, so stop reading after the n-th bit.
/// Because each packet has to be parsed to determine it's length and type,
/// We can't split the string somewhere.
/// This feels very much like we need to use a type of recursion that keeps track
/// of a cursor of some kind. This would allow us to continue after gathering subpackets,
/// from the point where the new cursor resides.
///
/// Therefore there are three ways to parse the input:
/// - expect a single packet (stop after we found it)
/// - expect n subpackets (stop after we found n)
/// - expect subpackets in n bits (stop after we reached n bits)
///
/// Lets create an enum for that!
#[derive(Debug)]
enum ParseMode {
    Single,
    Subpackets(usize),
    SubpacketsInBits(usize),
}
///
/// Because we chose Rust, we have to live with the fact that ownership problems
/// could occur when we pass slices around to recursive calls. Of course we don't need
/// mutable borrows, so we might just get away with it.
///
/// I think it's wise to opt for never storing the raw data in the packet, only
/// the parsed representations and the subpackets.
///
/// For part A, we can determine two packet types, one holds a literal value, and one
/// is an operator that holds a list of subpackets
/// Lets go ahead and create an enum for the packet types.
///
/// For part B, the operator packet is a bit more complicated.
/// It also contains an operator type.
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, Clone)]
enum PacketType {
    LiteralValue(u64),
    Operator((OperatorType, Vec<Packet>)),
}

/// The packet
#[derive(Debug, Clone)]
struct Packet {
    /// Each packet (or subpacket) has a version
    version: u8,
    /// Each packet has a type that holds the rest of the data
    /// (r# notation escapes the name so it doesnt clash with reserved keyword 'type')
    r#type: PacketType,
}

impl Packet {
    /// lets employ a simple wrapped constructor that can be passed a hex string
    fn from_hex(hex: &str) -> Packet {
        let bits = hex_to_binstr(hex);
        let mut cursor = 0;
        let packets = Packet::parse(&bits, ParseMode::Single, &mut cursor);
        // return the first packet in the vector
        packets.first().unwrap().clone()
    }

    /// Here is where the actual parsing should happen.
    /// This function is called for multiple parse modes. Let's figure out the scenarios.
    /// In any case, we get passed a slice of bits, and a parse mode.
    /// In the ParseMode::Single case, we expect a single packet.
    /// In the ParseMode::Subpackets case, we expect n subpackets, so this function
    /// should really return a vector of packets.
    fn parse(bits: &str, parse_mode: ParseMode, cursor: &mut usize) -> Vec<Packet> {
        let start_cursor = *cursor;

        println!(
            "[{} / {}] Start Parsing {:?}",
            cursor,
            bits.len(),
            parse_mode
        );

        // assume we can just start reading a packet, because there is at least one packet.
        let mut packets = Vec::new();
        let mut done = false;

        while !done {
            // read one packet and advance the cursor accordingly
            let packet = Packet::read_packet(bits, cursor);
            packets.push(packet);

            // check if we are done
            match parse_mode {
                ParseMode::Single => {
                    done = true;
                }
                ParseMode::Subpackets(n) => {
                    done = packets.len() == n;
                }
                ParseMode::SubpacketsInBits(n) => {
                    done = *cursor >= (n + start_cursor);
                }
            }
            println!(
                "[{} / {}] {:?} {} packets, done: {}",
                cursor,
                bits.len(),
                parse_mode,
                packets.len(),
                done
            );
        }

        packets
    }

    /// Reads a single packet from the string, and advances the cursor.
    fn read_packet(bits: &str, cursor: &mut usize) -> Packet {
        println!("[{} / {}] Start Reading Packet", cursor, bits.len());

        // First we need to read the version and type_id.
        let version = u8::from_str_radix(&bits[*cursor..*cursor + 3], 2).unwrap();
        *cursor += 3;
        let type_id = u8::from_str_radix(&bits[*cursor..*cursor + 3], 2).unwrap();
        *cursor += 3;

        println!(
            "[{} / {}] version: {}, type_id: {}",
            cursor,
            bits.len(),
            version,
            type_id
        );

        if type_id == 4 {
            // if the type_id is 4, we have a literal value
            let value = Packet::read_literal_value(bits, cursor);
            println!("[{} / {}] literal value: {}", cursor, bits.len(), value);
            Packet {
                version,
                r#type: PacketType::LiteralValue(value),
            }
        } else {
            let operator_type = match type_id {
                0 => OperatorType::Sum,
                1 => OperatorType::Product,
                2 => OperatorType::Minimum,
                3 => OperatorType::Maximum,
                5 => OperatorType::GreaterThan,
                6 => OperatorType::LessThan,
                7 => OperatorType::EqualTo,
                _ => panic!("Unknown operator type: {}", type_id),
            };

            // if the type_id is different from 4, we have an operator
            // take the byte at the cursor to determine length type
            let length_type = bits[*cursor..*cursor + 1].parse::<u8>().unwrap();
            println!("[{} / {}] length_type: {}", cursor, bits.len(), length_type);
            *cursor += 1;
            let subpackets = match length_type {
                0 => {
                    // If the length type ID is 0, then the next 15 bits are a number
                    // that represents the total length in bits of the sub-packets
                    // contained by this packet.
                    let length = usize::from_str_radix(&bits[*cursor..*cursor + 15], 2).unwrap();
                    println!(
                        "[{} / {}] subpackets in {} bits",
                        cursor,
                        bits.len(),
                        length
                    );
                    *cursor += 15;
                    Packet::parse(bits, ParseMode::SubpacketsInBits(length), cursor)
                }
                1 => {
                    // If the length type ID is 1, then the next 11 bits are a number
                    // that represents the number of sub-packets immediately contained
                    // by this packet.
                    let nr_packets =
                        usize::from_str_radix(&bits[*cursor..*cursor + 11], 2).unwrap();
                    println!("[{} / {}] {} subpackets", cursor, bits.len(), nr_packets);
                    *cursor += 11;
                    Packet::parse(bits, ParseMode::Subpackets(nr_packets), cursor)
                }
                _ => {
                    panic!("Unknown length type");
                }
            };
            Packet {
                version,
                r#type: PacketType::Operator((operator_type, subpackets)),
            }
        }
    }

    fn read_literal_value(bits: &str, cursor: &mut usize) -> u64 {
        // create a string to hold the literal value
        let mut literal_value = String::new();

        while *cursor + 5 <= bits.len() {
            // take four bits from start + 1 and add these to the literal value
            literal_value.push_str(&bits[(*cursor + 1)..(*cursor + 5)]);

            // if the bit at the cursor is a zero, break
            if bits[*cursor..*cursor + 1].starts_with('0') {
                // move cursor 5 places over
                *cursor += 5;
                break;
            }

            // otherwise, increment cursor by 5
            *cursor += 5;
        }

        // return the decimal representation of the binary string literal_value
        u64::from_str_radix(&literal_value, 2).unwrap()
    }

    /// returns the nested total of versions
    fn version_sum(&self) -> u32 {
        let mut sum = self.version as u32;
        if let PacketType::Operator((_, subpackets)) = &self.r#type {
            for packet in subpackets {
                sum += packet.version_sum();
            }
        }
        sum
    }

    // returns the value of the expression in the packet
    fn value(&self) -> u64 {
        match &self.r#type {
            PacketType::LiteralValue(value) => *value,
            PacketType::Operator((operator_type, subpackets)) => match operator_type {
                OperatorType::Sum => subpackets
                    .iter()
                    .fold(0, |acc, packet| acc + packet.value()),
                OperatorType::Product => subpackets
                    .iter()
                    .fold(1, |acc, packet| acc * packet.value()),
                OperatorType::Minimum => subpackets.iter().fold(std::u64::MAX, |acc, packet| {
                    std::cmp::min(acc, packet.value())
                }),
                OperatorType::Maximum => subpackets.iter().fold(std::u64::MIN, |acc, packet| {
                    std::cmp::max(acc, packet.value())
                }),
                OperatorType::GreaterThan => {
                    if subpackets[0].value() > subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::LessThan => {
                    if subpackets[0].value() < subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::EqualTo => {
                    if subpackets[0].value() == subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

fn hex_to_binstr(hex: &str) -> String {
    let mut binstr = String::new();
    for c in hex.chars() {
        let bin = match c.to_ascii_uppercase() {
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
            _ => panic!("Invalid hex character: {}", c),
        };
        binstr.push_str(bin);
    }
    binstr
}

pub fn main() {
    let lines = read_strs("input/day16.txt");
    let packet = Packet::from_hex(&lines[0]);
    println!("Version sum: {}", packet.version_sum());
    println!("Expression value: {}", packet.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_binstr() {
        assert_eq!(hex_to_binstr("D2FE28"), "110100101111111000101000");
    }

    #[test]
    fn test_version() {
        let packet = Packet::from_hex("D2FE28");
        assert_eq!(packet.version, 6);
    }

    #[test]
    fn test_literal_value() {
        let packet = Packet::from_hex("D2FE28");
        // match on packet.type
        if let PacketType::LiteralValue(value) = packet.r#type {
            assert_eq!(value, 2021);
        }
    }

    #[test]
    fn test_literal_value_parsing() {
        let mut cursor: usize = 28;
        let bits = hex_to_binstr("38006F45291200");
        let lit_val = Packet::read_literal_value(&bits, &mut cursor);
        assert_eq!(lit_val, 10);
        // cursor should be at 33
        assert_eq!(cursor, 33);

        let mut cursor = 39;
        let lit_val = Packet::read_literal_value(&bits, &mut cursor);
        assert_eq!(lit_val, 20);
        // cursor should be at 49
        assert_eq!(cursor, 49);
    }

    #[test]
    fn test_operator() {
        let packet = Packet::from_hex("38006F45291200");

        // packet version should be 1
        assert_eq!(packet.version, 1);

        // match on packet.type
        if let PacketType::Operator((_, subpackets)) = packet.r#type {
            // subpackets should have 2 items
            assert_eq!(subpackets.len(), 2);

            // subpacket[0] should be a literal value of 10
            if let PacketType::LiteralValue(value) = subpackets[0].r#type {
                assert_eq!(value, 10);
            } else {
                panic!("subpackets[0] is not a literal value");
            }

            // subpacket[1] should be a literal value of 20
            if let PacketType::LiteralValue(value) = subpackets[1].r#type {
                assert_eq!(value, 20);
            } else {
                panic!("subpackets[1] is not a literal value");
            }
        }
    }

    #[test]
    fn test_operator_2() {
        let packet = Packet::from_hex("EE00D40C823060");

        // packet version should be 7
        assert_eq!(packet.version, 7);

        // match on packet.type
        if let PacketType::Operator((_, subpackets)) = packet.r#type {
            // subpackets should have 2 items
            assert_eq!(subpackets.len(), 3);

            // subpacket[0] should be a literal value of 1
            if let PacketType::LiteralValue(value) = subpackets[0].r#type {
                assert_eq!(value, 1);
            } else {
                panic!("subpackets[0] is not a literal value");
            }

            // subpacket[1] should be a literal value of 2
            if let PacketType::LiteralValue(value) = subpackets[1].r#type {
                assert_eq!(value, 2);
            } else {
                panic!("subpackets[1] is not a literal value");
            }

            // subpacket[2] should be a literal value of 3
            if let PacketType::LiteralValue(value) = subpackets[2].r#type {
                assert_eq!(value, 3);
            } else {
                panic!("subpackets[2] is not a literal value");
            }
        }
    }

    #[test]
    fn test_version_sum() {
        let packet = Packet::from_hex("D2FE28");
        assert_eq!(packet.version_sum(), 6);

        let packet = Packet::from_hex("8A004A801A8002F478");
        assert_eq!(packet.version_sum(), 16);

        let packet = Packet::from_hex("620080001611562C8802118E34");
        assert_eq!(packet.version_sum(), 12);

        let packet = Packet::from_hex("C0015000016115A2E0802F182340");
        assert_eq!(packet.version_sum(), 23);

        let packet = Packet::from_hex("A0016C880162017C3686B18A3D4780");
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn test_expressions() {
        let packet = Packet::from_hex("C200B40A82");
        assert_eq!(packet.value(), 3);

        let packet = Packet::from_hex("04005AC33890");
        assert_eq!(packet.value(), 54);

        let packet = Packet::from_hex("880086C3E88112");
        assert_eq!(packet.value(), 7);

        let packet = Packet::from_hex("CE00C43D881120");
        assert_eq!(packet.value(), 9);

        let packet = Packet::from_hex("D8005AC2A8F0");
        assert_eq!(packet.value(), 1);

        let packet = Packet::from_hex("F600BC2D8F");
        assert_eq!(packet.value(), 0);

        // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
        let packet = Packet::from_hex("9C005AC2F8F0");
        assert_eq!(packet.value(), 0);

        // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2.
        let packet = Packet::from_hex("9C0141080250320F1802104A08");
        assert_eq!(packet.value(), 1);
    }
}
