//! Macros for rinux.

#![no_std]

#![warn(unused)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]


extern crate alloc;
use alloc::{format,string::{ToString,String}};
extern crate proc_macro;
use proc_macro as pm;



/**
Species the entry point of the kernel.

Example:
```rust
#[rinuxcore::main]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rinuxcore::init(&boot_info);// Initializes Rinux
    println!("Hello World");
}
```
*/
#[proc_macro_attribute]
pub fn main(args: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    if args.to_string() != "" {
        
    }
    let mut old = (&item).to_string();
    old.push_str("");
    let mut iter = item.clone().into_iter();
    let fn_name: String = match iter.nth(1) {
        Some(v) => v.to_string(),
        _ => String::from("__fname")
    };
    format!(
        r#"{oldfn}
macro_rules! __kernel {{
    ($path:path) => {{
        #[doc(hidden)]
        #[export_name = "_start"]
        pub extern "C" fn __impl_start(boot_info: &'static $crate::BootInfo) -> ! {{
            let f: fn(&'static $crate::BootInfo) -> ! = $path;
            f(boot_info)
        }}
    }};
}}
fn __start(b:&'static crate::BootInfo) -> ! {{
    unsafe {{ rinuxcore::__core_init() }};
    rinuxcore::vga_buffer::__init_rinux();
    {fname}(b)
}}
__kernel!(__start);
"#,
        fname=fn_name,
        oldfn=old
    ).parse().expect("Failed to parse macro main")
}
