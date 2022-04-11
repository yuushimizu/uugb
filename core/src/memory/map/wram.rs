use super::Segment;

const PRIMARY: Segment = Segment::Leaf(
    |components, address| components.wram.read(address),
    |components, address, value| components.wram.write(address, value),
);

const BANK: Segment = Segment::Leaf(
    |components, address| components.wram.read_bank(address),
    |components, address, value| components.wram.write_bank(address, value),
);

pub const WRAM: Segment = {
    Segment::Nested(|address| match address {
        0xC000..=0xCFFF => &Segment::Offset(0xC000, &PRIMARY),
        0xD000..=0xDFFF => &Segment::Offset(0xD000, &BANK),
        0xE000..=0xEFFF => &Segment::Offset(0xE000, &PRIMARY),
        0xF000..=0xFDFF => &Segment::Offset(0xF000, &BANK),
        0xFF70 => &Segment::Leaf(
            |components, _| components.wram.bank_switch(),
            |components, _, value| components.wram.set_bank_switch(value),
        ),
        _ => unreachable!(),
    })
};
