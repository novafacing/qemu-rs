use darling::{ast::NestedMeta, Error, FromMeta, Result};
use proc_macro::TokenStream;
use quote::quote;
use semver::{Version, VersionReq};
use syn::{parse_macro_input, ItemImpl};

#[derive(Debug, FromMeta)]
struct QemuVersionOpts {
    requirement: String,
}

impl QemuVersionOpts {
    fn generate(&self, input: &ItemImpl) -> Result<TokenStream> {
        let all_versions = [
            Version::new(4, 2, 0),
            Version::new(6, 0, 0),
            Version::new(9, 0, 0),
            Version::new(9, 1, 0),
            Version::new(9, 2, 0),
            Version::new(10, 1, 0),
        ];

        let requirement = VersionReq::parse(&self.requirement)
            .map_err(|e| Error::custom(format!("Failed to parse version requirement: {e}")))?;
        let versions = all_versions
            .iter()
            .enumerate()
            .filter(|(_, v)| requirement.matches(v))
            .map(|(i, _)| {
                let feature = format!("plugin-api-v{i}");
                quote!(feature = #feature)
            })
            .collect::<Vec<_>>();

        if versions.is_empty() {
            return Err(Error::custom(format!(
                "No supported QEMU versions match requirement '{}'",
                self.requirement
            )));
        }

        Ok(TokenStream::from(quote! {
            #[cfg(any(
                #(#versions),*
            ))]
            #input
        }))
    }
}

#[proc_macro_attribute]
/// Attribute translates the provided QEMU version restrictions (e.g. ">= 5.2.0,
/// < 7.0.0") into the appropriate CFG attributes
pub fn qemu_version(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let meta = match NestedMeta::parse_meta_list(attrs.into()) {
        Ok(meta) => meta,
        Err(err) => return TokenStream::from(Error::from(err).write_errors()),
    };

    let options = match QemuVersionOpts::from_list(&meta) {
        Ok(options) => options,
        Err(err) => return TokenStream::from(err.write_errors()),
    };

    let impl_item = parse_macro_input!(item as ItemImpl);

    options
        .generate(&impl_item)
        .unwrap_or_else(|e| TokenStream::from(e.write_errors()))
}
