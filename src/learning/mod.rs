#![allow(unused)]
mod circularref;
mod polymorphism;
mod traits;
mod weakcircularref;

pub use circularref::test_circular_ref;
pub use polymorphism::test_polymorphism;
pub use traits::test_traits;
pub use weakcircularref::test_weak_circular_ref;
