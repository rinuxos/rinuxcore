#![no_std]


#[doc(hidden)]
use core::fmt::{
    Result as FmtResult,
    Formatter,
    Debug
};



#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Job {
    _internal: fn() -> (),
}
impl Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Job {{ ... }}")
    }
}
impl Job {
    pub fn new(func: fn() -> ()) -> Self {
        Self { _internal: func }
    }
    pub fn run(&self) {
        (self._internal)()
    }
}


#[macro_export(local_inner_macros)]
macro_rules! job {
    ( $name: ident ) => { job!($name=||{core::panic!("Hello World!")}); };
    ( $name: ident = $body: expr ) => {
        rinuxcore::composer::__lazy!(
            static ref func: fn() -> () = $body;
            static ref $name: rinuxcore::composer::Job = rinuxcore::composer::Job::new(*func);
        );
    };
    () => ()
}


/// Nedded for the macro
#[doc(hidden)]
pub use lazy_static::lazy_static as __lazy;
