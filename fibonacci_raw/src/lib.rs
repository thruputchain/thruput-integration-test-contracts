#![no_std]

#[allow(unused_imports)]
use sdk::*;

pub fn fibonacci_slow(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    if count == 1 {
        return 1;
    }
    fibonacci_slow(count - 1) + fibonacci_slow(count - 2)
}

#[unsafe(export_name = "1")]
pub extern "C" fn fibonacci_slow_raw_1() -> () {
    fibonacci_slow(1);
}

#[unsafe(export_name = "5")]
pub extern "C" fn fibonacci_slow_raw_5() -> () {
    fibonacci_slow(5);
}

#[unsafe(export_name = "A")]
pub extern "C" fn fibonacci_slow_raw_10() -> () {
    fibonacci_slow(10);
}

#[unsafe(export_name = "F")]
pub extern "C" fn fibonacci_slow_raw_15() -> () {
    fibonacci_slow(15);
}

#[unsafe(export_name = "14")]
pub extern "C" fn fibonacci_slow_raw_20() -> () {
    fibonacci_slow(20);
}

#[unsafe(export_name = "19")]
pub extern "C" fn fibonacci_slow_raw_25() -> () {
    fibonacci_slow(25);
}

#[unsafe(export_name = "FFFFFFFFFFFFFFFF")]
pub extern "C" fn startup_null() -> () {
    ()
}
