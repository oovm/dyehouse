#![forbid(missing_docs)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_doc_code_examples)]
#![allow(clippy::derivable_impls)]
#![doc = include_str!("../readme.md")]

// pub use self::overlap::{iter::CodeViewIter, CodeSpan2, CodeRender2};
pub mod no_overlap;
pub mod overlap;
