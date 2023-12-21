//! Core traits and functions for batch and stream hash computation.
//!
//! # Setup
//!
//! To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:
//!
//! ```toml
//! [dependencies]
//! chksum-hash-core = "0.0.0"
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```sh
//! cargo add chksum-hash-core
//! ```     
//!
//! # Example Crates
//!
//! For implementation-specific examples, refer to the source code of the following crates:
//!
//! * [`chksum-hash-md5`](https://docs.rs/chksum-hash-md5/)
//! * [`chksum-hash-sha1`](https://docs.rs/chksum-hash-sha1/)
//! * [`chksum-hash-sha2`](https://docs.rs/chksum-hash-sha2/)
//!     * [`chksum-hash-sha2-224`](https://docs.rs/chksum-hash-sha2-224)
//!     * [`chksum-hash-sha2-256`](https://docs.rs/chksum-hash-sha2-256)
//!     * [`chksum-hash-sha2-384`](https://docs.rs/chksum-hash-sha2-384)
//!     * [`chksum-hash-sha2-512`](https://docs.rs/chksum-hash-sha2-512)
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

use std::fmt::{Display, LowerHex, UpperHex};

/// Creates a default hash.
#[must_use]
pub fn default<U>() -> U
where
    U: Update,
{
    Default::default()
}

/// Computes the hash of the given input.
pub fn hash<U>(data: impl AsRef<[u8]>) -> U::Digest
where
    U: Update,
{
    let mut hash = default::<U>();
    hash.update(data);
    hash.digest()
}

/// A trait for hash objects in an in-progress state.
///
/// This trait defines the common interface for hash objects that are in the process
/// of being updated with new data. Implementations of this trait typically maintain
/// an internal state that evolves as new data is added.
pub trait Update: Default {
    /// The type representing the digest produced by finalizing the hash.
    type Digest: Digest;

    /// The type representing the finalized state, which can be converted into a digest.
    type Finalize: Finalize<Digest = Self::Digest, Update = Self>;

    /// Updates the hash state with an input data.
    fn update(&mut self, data: impl AsRef<[u8]>);

    /// Finalizes the hash state.
    fn finalize(&self) -> Self::Finalize;

    /// Resets the hash state to its initial state.
    fn reset(&mut self);

    /// Produces the hash digest using internal finalization.
    fn digest(&self) -> Self::Digest {
        self.finalize().digest()
    }
}

/// A trait for hash objects in a finalized state.
///
/// This trait defines the common interface for hash objects that have been finalized,
/// producing the final digest. Implementations of this trait typically provide methods
/// for obtaining the final digest and resetting the state to perform additional hashing.
pub trait Finalize {
    /// The type representing the digest produced by finalizing the hash.
    type Digest: Digest;

    /// The type representing the in-progress state, which can be finalized again.
    type Update: Update;

    /// Produces the final digest.
    fn digest(&self) -> Self::Digest;

    /// Resets the finalized state to the in-progress state.
    fn reset(&self) -> Self::Update;
}

/// A trait for hash digests.
///
/// This trait defines the common interface for hash digests, providing methods for
/// converting the digest into various representations.
pub trait Digest: Display {
    /// Returns the digest as a byte slice.
    fn as_bytes(&self) -> &[u8]
    where
        Self: AsRef<[u8]>,
    {
        self.as_ref()
    }

    /// Returns the digest as a lowercase hexadecimal string.
    fn to_hex_lowercase(&self) -> String
    where
        Self: LowerHex,
    {
        format!("{self:x}")
    }

    /// Returns the digest as an uppercase hexadecimal string.
    fn to_hex_uppercase(&self) -> String
    where
        Self: UpperHex,
    {
        format!("{self:X}")
    }
}
