#![no_main]

use libfuzzer_sys::fuzz_target;
use proc_macro2::Span;
use std::str;
use syn_send::parse::{ParseStream, Parser};

fn immediate_fail(_input: ParseStream) -> syn_send::Result<()> {
    Err(syn_send::Error::new(Span::call_site(), ""))
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 300 {
        if let Ok(string) = str::from_utf8(data) {
            let _ = immediate_fail.parse_str(string);
        }
    }
});
