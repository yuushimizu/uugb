pub mod destination;
pub mod indirection;
pub mod literal;
pub mod register;
pub mod source;

pub use destination::{Destination16, Destination8, Writer16, Writer8};
pub use literal::LITERAL;
pub use source::{Source16, Source8};
