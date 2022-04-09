use super::{Segment, UNKNOWN};

pub const WRAM: Segment = {
    Segment::Nested(|address| match address {
        0xC000..=0xCFFF => &Segment::Offset(
            0xC000,
            &Segment::Leaf(
                |components, address| components.wram.read(address),
                |components, address, value| components.wram.write(address, value),
            ),
        ),
        0xD000..=0xDFFF => &Segment::Offset(
            0xD000,
            &Segment::Leaf(
                |components, address| components.wram.read_bank(address),
                |components, address, value| components.wram.write_bank(address, value),
            ),
        ),
        0xFF70 => &Segment::Leaf(
            |components, _| components.wram.bank_switch(),
            |components, _, value| components.wram.set_bank_switch(value),
        ),
        _ => &UNKNOWN,
    })
};
