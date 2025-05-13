use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let f_ident = &f.sig.ident;

    let arch = match () {
        #[cfg(feature = "armv8a")]
        () => quote!(armv8a),
    };

    quote!(
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn _start() -> ! {
            ::core::arch::naked_asm!("b {}", sym #arch::start::<crate::EntryImpl>)
        }

        struct EntryImpl;
        impl #arch::Entry for EntryImpl {
            unsafe extern "C" fn entry() -> ! {
                #f_ident()
            }
        }

        #f
    )
    .into()
}
