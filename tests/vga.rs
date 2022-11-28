#![no_std]
#![no_main]

#![feature(plugin)]



use rinuxcore::{
    BootInfo,
    println
};



#[rinuxcore::main]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rinuxcore::test_runner(&[&test_println]);
    rinuxcore::hlt_loop();
}


fn test_println() {
    println!("test_println output");
}

fn test_println_many() {
    for id in 0..200 {
        println!("printing line: {}",&id);
    }
}

#[panic_handler]
fn panic(info: &std3::panic::PanicInfo) -> ! {
    rinuxcore::print_err!("{}", info);
    rinuxcore::hlt_loop();
}