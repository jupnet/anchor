/// Crate a default [`anchor_lang::IdlBuild`] implementation for the given type.
///
/// This is used in order to make wrapper accounts of `anchor-spl` work with `idl-build` feature.
macro_rules! impl_idl_build {
    ($ty: ty) => {
        impl anchor_lang::IdlBuild for $ty {}

        // This is not used for the IDL generation since default `IdlBuild` impl doesn't include
        // the type in the IDL but it stil needs to be added in order to make compilation work.
        //
        // TODO: Find a better way to handle discriminators of wrapped external accounts.
        impl anchor_lang::Discriminator for $ty {
            const DISCRIMINATOR: &'static [u8] = &[];
        }
    };
}

#[cfg(feature = "metadata")]
impl_idl_build!(crate::metadata::MetadataAccount);
#[cfg(feature = "metadata")]
impl_idl_build!(crate::metadata::MasterEditionAccount);
#[cfg(feature = "metadata")]
impl_idl_build!(crate::metadata::TokenRecordAccount);

#[cfg(feature = "stake")]
impl_idl_build!(crate::stake::StakeAccount);

// Temporarily unsupported in the Jupnet adapter. Keep `token.rs` intact and out of the build.
// impl_idl_build!(crate::token::Mint);
// impl_idl_build!(crate::token::TokenAccount);

#[cfg(any(feature = "token_2022", feature = "token_2022_extensions"))]
impl_idl_build!(crate::token_interface::Mint);
#[cfg(any(feature = "token_2022", feature = "token_2022_extensions"))]
impl_idl_build!(crate::token_interface::TokenAccount);
