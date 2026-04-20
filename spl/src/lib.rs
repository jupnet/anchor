#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Anchor CPI wrappers for popular programs in the Solana ecosystem.

#[cfg(feature = "associated_token")]
pub mod associated_token;

#[cfg(feature = "mint")]
pub mod mint;

// Temporarily unsupported in the Jupnet adapter. Keep `token.rs` intact and out of the build.
// #[cfg(feature = "token")]
// pub mod token;

// Temporarily unsupported in the Jupnet adapter. Keep `token_2022.rs` intact and out of the build.
// #[cfg(feature = "token_2022")]
// pub mod token_2022;

#[cfg(any(feature = "token_2022", feature = "token_2022_extensions"))]
pub mod jpl_token_2022;

// Compile the Jupnet-specific copy while leaving the upstream
// `token_2022_extensions/` sources untouched in the tree.
#[cfg(feature = "token_2022_extensions")]
pub mod jpl_token_2022_extensions;

// Keep the upstream public path stable while routing it to the Jupnet-specific copy.
#[cfg(feature = "token_2022_extensions")]
pub use jpl_token_2022_extensions as token_2022_extensions;

#[cfg(any(feature = "token_2022", feature = "token_2022_extensions"))]
pub mod token_interface;

#[cfg(feature = "governance")]
pub mod governance;

#[cfg(feature = "stake")]
pub mod stake;

// Temporarily unsupported in the Jupnet adapter. Keep `metadata.rs` intact and out of the build.
// #[cfg(feature = "metadata")]
// pub mod metadata;

#[cfg(feature = "memo")]
pub mod memo;

#[cfg(feature = "idl-build")]
mod idl_build;
