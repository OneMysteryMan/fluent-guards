//! Provides various functions to guard your code.

mod guard;
mod guards;

pub use guard::Guard;
pub use guards::Guards;

pub enum Bound {
	Inclusive,
	Exclusive,
}
