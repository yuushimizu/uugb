use crate::util::ascii;
use std::{fmt, ops::RangeInclusive};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Licensee {
    old_code: u8,
    new_code: Option<Vec<u8>>,
}

const OLD_ADDRESS: usize = 0x014B;

const NEW_RANGE: RangeInclusive<usize> = 0x0144..=0x0145;

fn name_from_old_code(code: u8) -> &'static str {
    match code {
        0x00 => "None",
        0x01 => "Nintendo",
        0x08 => "CAPCOM",
        0x09 => "Hot-B",
        0x0A => "JALECO",
        0x0B => "Coconuts",
        0x0C => "Elite Systems",
        0x13 => "Electronic Arts",
        0x18 => "Hudson Soft",
        0x19 => "ITC Entertainment",
        0x1A => "YANOMAN",
        0x1D => "Clary",
        0x1F => "Virgin",
        0x24 => "PCM Complete",
        0x25 => "San-x",
        0x28 => "Kotobuki Systems",
        0x29 => "Seta",
        0x30 => "Infogrames",
        0x31 => "Nintendo",
        0x32 => "BANDAI",
        0x34 => "KONAMI",
        0x35 => "Hector",
        0x38 => "CAPCOM",
        0x39 => "BANPRESTO",
        0x3C => "Entertainment i...",
        0x3E => "Gremlin",
        0x41 => "Ubisoft",
        0x42 => "ATLUS",
        0x44 => "MALIBU",
        0x46 => "Angel",
        0x47 => "Spectrum HoloByte",
        0x49 => "IREM",
        0x4A => "Virgin",
        0x4D => "MALIBU",
        0x4F => "U.S. Gold",
        0x50 => "Absolute",
        0x51 => "Acclaim",
        0x52 => "Activision",
        0x53 => "American Sammy",
        0x54 => "GameTek",
        0x55 => "Park Place",
        0x56 => "LJN",
        0x57 => "MATCHBOX",
        0x59 => "Milton Bradley",
        0x5A => "Mindscape",
        0x5B => "Romstar",
        0x5C => "NAXAT Soft",
        0x5D => "Tradewest",
        0x60 => "Titus",
        0x61 => "Virgin",
        0x67 => "Ocean",
        0x69 => "Electronic Arts",
        0x6E => "Elite Systems",
        0x6F => "Electro Brain",
        0x70 => "Infogrames",
        0x71 => "Interplay",
        0x72 => "Broderbund",
        0x73 => "Sculptured Soft",
        0x75 => "The Sales Curve",
        0x78 => "THQ",
        0x79 => "Accolade",
        0x7A => "Triffix Entertainment",
        0x7C => "Microprose",
        0x7F => "KEMCO",
        0x80 => "Misawa Entertainment",
        0x83 => "LOZC",
        0x86 => "Tokumashoten Intermedia",
        0x8B => "Bullet-Proof Software",
        0x8C => "VIC TOKAI",
        0x8E => "APE",
        0x8F => "I'Max",
        0x91 => "CHUNSOFT",
        0x92 => "Video System",
        0x93 => "TSUBURAYA",
        0x95 => "Varie",
        0x96 => "YONEZAWA/S'PAL",
        0x97 => "KANEKO",
        0x99 => "ARC",
        0x9A => "Nihon Bussan",
        0x9B => "TECMO",
        0x9C => "Imagineer",
        0x9D => "BANPRESTO",
        0x9F => "NOVA",
        0xA1 => "HORI Electric",
        0xA2 => "BANDAI",
        0xA4 => "KONAMI",
        0xA6 => "Kawada",
        0xA7 => "TAKARA",
        0xA9 => "Technos Japan",
        0xAA => "Broderbund",
        0xAC => "TOEI Animation",
        0xAD => "TOHO",
        0xAF => "namco",
        0xB0 => "Acclaim",
        0xB1 => "Ascii | Nexoft",
        0xB2 => "BANDAI",
        0xB4 => "ENIX",
        0xB6 => "HAL",
        0xB7 => "SNK",
        0xB9 => "PONY CANYON",
        0xBA => "Culture Brain",
        0xBB => "SUNSOFT",
        0xBD => "Sony Imagesoft",
        0xBF => "Sammy",
        0xC0 => "TAITO",
        0xC2 => "KEMCO",
        0xC3 => "SQUARESOFT",
        0xC4 => "Tokumashoten Intermedia",
        0xC5 => "Data East",
        0xC6 => "TONKINHOUSE",
        0xC8 => "Koei",
        0xC9 => "UFL",
        0xCA => "Ultra",
        0xCB => "VAP",
        0xCC => "USE",
        0xCD => "Meldac",
        0xCE => "PONY CANYON",
        0xCF => "Angel",
        0xD0 => "TAITO",
        0xD1 => "SOFEL",
        0xD2 => "Quest",
        0xD3 => "Sigma Enterprises",
        0xD4 => "ASK Kodansha",
        0xD6 => "NAXAT Soft",
        0xD7 => "COPYA SYSTEM",
        0xD9 => "BANPRESTO",
        0xDA => "TOMY",
        0xDB => "LJN",
        0xDD => "NCS",
        0xDE => "Human",
        0xDF => "Altron",
        0xE0 => "JALECO",
        0xE1 => "Towachiki",
        0xE2 => "Yutaka",
        0xE3 => "Varie",
        0xE5 => "EPOCH",
        0xE7 => "Athena",
        0xE8 => "Asmik",
        0xE9 => "Natsume",
        0xEA => "KING RECORDS",
        0xEB => "Atlus",
        0xEC => "EPIC/Sony Records",
        0xEE => "IGS",
        0xF0 => "A Wave",
        0xF3 => "Extreme Entertainment",
        0xFF => "LJN",
        _ => "",
    }
}

fn name_from_new_code(code: &[u8]) -> &'static str {
    u8::from_str_radix(ascii::from_bytes(code).as_str(), 16).map_or("", |combined| match combined {
        0x19 => "B-AI",
        0x20 => "KSS",
        0x22 => "POW",
        0x28 => "KEMCO",
        0x30 => "Viacom",
        0x33 => "Ocean/Acclaim",
        0x37 => "TAITO",
        0x38 => "Hudson",
        0x47 => "Bullet-Proof",
        0x54 => "KONAMI",
        0x55 => "Hi Tech Entertainment",
        0x58 => "Mattel",
        0x64 => "LucasArts",
        0x75 => "SCI",
        0x87 => "Tsukuda Original",
        0x93 => "Ocean/Acclaim",
        0x99 => "PACK-IN-SOFT",
        combined => name_from_old_code(combined),
    })
}

impl Licensee {
    pub fn load(rom: &[u8]) -> Self {
        let old_code = rom[OLD_ADDRESS];
        Self {
            old_code,
            new_code: match old_code {
                0x33 => Some(rom[NEW_RANGE].into()),
                _ => None,
            },
        }
    }

    pub fn old_code(&self) -> u8 {
        self.old_code
    }

    pub fn new_code(&self) -> Option<&[u8]> {
        self.new_code.as_deref()
    }

    pub fn name(&self) -> &str {
        self.new_code.as_ref().map_or_else(
            || name_from_old_code(self.old_code),
            |new_code| name_from_new_code(new_code),
        )
    }
}

impl fmt::Display for Licensee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.new_code {
            Some(new_code) => write!(
                f,
                "{} ({:02X}/{})",
                self.name(),
                self.old_code,
                ascii::from_bytes(new_code)
            ),
            None => write!(f, "{} ({:02X})", self.name(), self.old_code),
        }
    }
}
