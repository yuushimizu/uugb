pub mod destination;
pub mod indirection;
pub mod literal;
pub mod register;
pub mod source;
pub mod stack_pointer;

pub use destination::{Destination, Writer};
pub use literal::LITERAL;
pub use source::Source;

pub type SourceRef<T> = &'static dyn Source<T>;
pub type DestinationRef<T> = &'static dyn Destination<T>;
