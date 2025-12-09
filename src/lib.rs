#[macro_use]
pub(crate) mod macros;
pub mod date_time;
pub mod error;

pub use error::{ArchiveError, Result};

pub mod ace;
pub mod arc;
pub mod arj;
pub mod ha;
pub mod hyp;
pub mod lha;
pub mod rar;
pub mod sevenz;
pub mod sq;
pub mod sqz;
pub mod uc2;
pub mod z;
pub mod zip;
pub mod zoo;

pub mod unified;
