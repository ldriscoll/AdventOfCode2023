use std::collections::{HashMap};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

struct PartNumber {
    part_number: u32,
    y_pos: usize,
    x_start_pos: usize,
    len: usize,
}

struct SymbolPos {
    symbol: char,
    y_pos: usize,
    x_pos: usize,
}

impl Clone for SymbolPos {
    fn clone(&self) -> Self {
        SymbolPos {
            symbol: self.symbol,
            y_pos: self.y_pos,
            x_pos: self.x_pos,
        }
    }
}

impl PartialEq<Self> for SymbolPos {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.y_pos == other.y_pos && self.x_pos == other.x_pos
    }
}

impl Eq for SymbolPos {}

impl Hash for SymbolPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        self.y_pos.hash(state);
        self.x_pos.hash(state);
    }
}

struct PartAndGears<'a> {
    part: &'a PartNumber,
    gears: Vec<SymbolPos>,
}

pub(crate) fn calculate_part_sum(filename: &str) -> u32 {
    let schematic = read_schematic(filename);
    let all_parts: Vec<PartNumber> = get_part_numbers(&schematic);
    let filtered_parts: Vec<&PartNumber> = all_parts
        .iter()
        .filter(|pn| !get_near_symbols(pn, &schematic).is_empty())
        .collect();

    let mut total = 0u32;
    for part in filtered_parts {
        println!("Part number {} start ({}, {}), len {}",
                 part.part_number,
                 part.y_pos,
                 part.x_start_pos,
                 part.len);
        total += part.part_number;
    }
    return total;
}

pub(crate) fn get_gear_ratios(filename: &str) -> u64 {
    let schematic = read_schematic(filename);
    let all_parts: Vec<PartNumber> = get_part_numbers(&schematic);
    let with_gears: Vec<PartAndGears> = all_parts
        .iter()
        .map(|pn| create_part_and_gears(pn, &schematic))
        .filter(|pag| !pag.gears.is_empty())
        .collect();

    let mut bygear: HashMap<SymbolPos, Vec<&PartNumber>> = HashMap::new();

    for with_gear in with_gears {
        println!("Part {} in pos {}, {} has {} gears", with_gear.part.part_number, with_gear.part.y_pos, with_gear.part.y_pos, with_gear.gears.len());
        for gear in with_gear.gears {
            let parts = bygear.entry(gear).or_insert(Vec::new());
            parts.push(with_gear.part);
        }
    }
    let mut total_ratios:u64 = 0;
    for gear in bygear {
        if gear.1.len() == 2 {
            let ratio = gear.1.iter().map(|pn| pn.part_number).fold(1, |acc, pn| acc * pn);
            println!("Gear at {}, {} has {} parts, ratio {}",
                     gear.0.y_pos, gear.0.x_pos, gear.1.len(), ratio);
            total_ratios += ratio as u64;
        }
    }
    total_ratios
}

fn create_part_and_gears<'a>(pn: &'a PartNumber, schematic: &Vec<String>) -> PartAndGears<'a> {
    let gears: Vec<SymbolPos> = get_near_symbols(&pn, schematic)
        .iter()
        .filter(|sym| sym.symbol == '*')
        .map(|sym| sym.clone())
        .collect();
    let pag: PartAndGears = PartAndGears {
        part: pn,
        gears: gears.to_vec(),
    };
    pag
}

fn get_near_symbols<'a>(part_number: &&PartNumber, schematic: &Vec<String>) -> Vec<SymbolPos> {
    let above: i32 = part_number.y_pos as i32 - 1;
    let below = part_number.y_pos as i32 + 2;
    let left: i32 = part_number.x_start_pos as i32 - 1;
    let right = part_number.x_start_pos + part_number.len + 1;

    println!("Part {} pos {},{}, testing range {},{} to {}, {}",
             part_number.part_number,
             part_number.y_pos,
             part_number.x_start_pos,
             above, left,
             below, right);
    let mut near_chars: Vec<SymbolPos> = Vec::new();
    for y_pos in (above..below) {
        println!("Testing y {}", y_pos);
        if (y_pos >= 0 && y_pos < schematic.len() as i32) {
            for x_pos in (left..right as i32) {
                println!("testing x {}", x_pos);
                if (x_pos >= 0) {
                    let char_at = schematic.get(y_pos as usize).unwrap().chars().nth(x_pos as usize);
                    println!("....Checking {},{} - {}", y_pos, x_pos, char_at.unwrap_or('E'));
                    if (char_at.is_none()) {
                        println!("***Char not set at {},{}", y_pos, x_pos);
                    } else if (is_symbol(char_at.unwrap())) {
                        println!("... is a part {} at {},{}",
                                 char_at.unwrap(),
                                 y_pos,
                                 x_pos);
                        let sym = SymbolPos {
                            symbol: char_at.unwrap(),
                            y_pos: y_pos as usize,
                            x_pos: x_pos as usize,
                        };
                        near_chars.push(sym);
                    }
                }
            }
        }
    }
    if (near_chars.is_empty()) {
        println!("... not a part");
    }
    near_chars.to_vec()
}

fn get_part_numbers(schematic: &Vec<String>) -> Vec<PartNumber> {
    let mut all_parts: Vec<PartNumber> = Vec::new();
    for line_num in (0..schematic.len()) {
        let mut others = get_part_numbers_from_line(schematic.get(line_num).unwrap(), line_num);
        all_parts.append(&mut others);
    }

    return all_parts;
}

fn get_part_numbers_from_line(line: &str, y_pos: usize) -> Vec<PartNumber> {
    let mut parts: Vec<PartNumber> = Vec::new();
    let mut part_number = 0u32;
    let mut x_start_pos = -1i32;
    let mut len = 0;

    let chars = line.chars().enumerate();

    for pos in chars {
        if pos.1.is_digit(10) {
            part_number = part_number * 10 + pos.1.to_digit(10).unwrap();
            len += 1;
            if (x_start_pos < 0) {
                x_start_pos = pos.0 as i32;
            }
        } else if part_number > 0 {
            parts.push(
                PartNumber {
                    part_number,
                    y_pos,
                    x_start_pos: x_start_pos as usize,
                    len,
                }
            );
            part_number = 0;
            x_start_pos = -1;
            len = 0;
        }
    }
    if part_number > 0 {
        parts.push(
            PartNumber {
                part_number,
                y_pos,
                x_start_pos: x_start_pos as usize,
                len,
            }
        );
    }

    return parts;
}

fn read_schematic(filename: &str) -> Vec<String> {
    return read_to_string(filename).unwrap().lines().map(str::to_string).collect();
}


fn is_symbol(character: char) -> bool {
    return !character.is_digit(10)
        && character != '.';
}