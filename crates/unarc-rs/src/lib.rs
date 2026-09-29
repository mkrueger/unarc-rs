#![forbid(unsafe_code)]

#[macro_use]
pub(crate) mod macros;
pub mod date_time;
pub mod encryption;
pub mod error;
pub mod limits;

pub use encryption::{EncryptionMethod, RarEncryption, SevenZEncryption, ZipEncryption};
pub use error::{ArchiveError, Result};
pub use limits::DEFAULT_MAX_ENTRY_SIZE;

pub mod ace;
pub mod arc;
pub mod arj;
pub mod bz2;
pub mod gz;
pub mod ha;
pub mod hyp;
pub mod ice;
pub mod jar;
pub mod lha;
pub mod packice;
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
pub use unified::{ArchiveOptions, VolumeProvider};

// Password verifiers are shared across threads (e.g. with rayon) when testing passwords in parallel.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ace::AcePasswordVerifier>();
    assert_send_sync::<arc::password_verifier::ArcPasswordVerifier>();
    assert_send_sync::<arj::password_verifier::ArjPasswordVerifier>();
    assert_send_sync::<rar::RarPasswordVerifier>();
    assert_send_sync::<sevenz::SevenZPasswordVerifier>();
    assert_send_sync::<zip::ZipPasswordVerifier>();
};
