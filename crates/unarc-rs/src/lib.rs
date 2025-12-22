#[macro_use]
pub(crate) mod macros;
pub mod date_time;
pub mod encryption;
pub mod error;

pub use encryption::{EncryptionMethod, RarEncryption, SevenZEncryption, ZipEncryption};
pub use error::{ArchiveError, Result};

pub mod ace;
pub mod arc;
pub mod arj;
pub mod bz2;
pub mod gz;
pub mod ha;
pub mod hyp;
pub mod ice;
pub mod lha;
pub mod rar;
pub mod sevenz;
pub mod sq;
pub mod sqz;
pub mod tar;
pub mod tarz;
pub mod tbz;
pub mod tgz;
pub mod uc2;
pub mod z;
pub mod zip;
pub mod zoo;

pub mod unified;
pub use unified::ArchiveOptions;
