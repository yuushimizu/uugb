use crate::memory::{ComponentsRefs, ComponentsRefsMut};

pub enum Segment<'a> {
    Leaf(
        fn(&ComponentsRefs, u16) -> u8,
        fn(&mut ComponentsRefsMut, u16, u8),
    ),
    Nested(fn(address: u16) -> &'a Segment<'a>),
    Offset(u16, &'a Segment<'a>),
}

impl<'a> Segment<'a> {
    pub fn read(&self, components: &ComponentsRefs, address: u16) -> u8 {
        use Segment::*;
        match self {
            Leaf(reader, _) => reader(components, address),
            Nested(inner) => inner(address).read(components, address),
            Offset(offset, inner) => inner.read(components, address - offset),
        }
    }

    pub fn write(&self, components: &mut ComponentsRefsMut, address: u16, value: u8) {
        use Segment::*;
        match self {
            Leaf(_, writer) => writer(components, address, value),
            Nested(inner) => inner(address).write(components, address, value),
            Offset(offset, inner) => inner.write(components, address - offset, value),
        }
    }
}
