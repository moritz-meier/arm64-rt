use darling::{Error, FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let f = parse_macro_input!(input as ItemFn);
    let f_ident = &f.sig.ident;

    let arch = match () {
        #[cfg(feature = "arm64")]
        () => quote!(arm64),
    };

    let exceptions = if let Some(excps) = args.exceptions {
        quote!(crate::#excps)
    } else {
        quote!(#arch::DefaultExceptions)
    };

    quote!(
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".text.start")]
        pub unsafe extern "C" fn _start() -> ! {
            ::core::arch::naked_asm!("b {}", sym #arch::start::<crate::EntryImpl, #exceptions>)
        }

        struct EntryImpl;
        impl #arch::Entry for EntryImpl {
            unsafe extern "C" fn entry(info: EntryInfo) -> ! {
                #f_ident(info)
            }
        }

        #f
    )
    .into()
}

#[derive(Debug, FromMeta)]
struct MacroArgs {
    exceptions: Option<Ident>,
}
