// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 2381d783e8135e959527afe5c866b659c118f2f5

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};

#[cfg(feature="xdrs")]
use ::xdrs::{ReadXDR, WriteXDR};


use xdr_common::*;


#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct Foo {
    pub a: u32,
}

// test cases from xdrtest.x

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum senum {
    SE_NEGATIVE = -1,
    SE_POSITIVE = 1,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
pub enum sunion {
    // case SE_NEGATIVE:
    //   bool neg;
    SE_NEGATIVE { neg: bool } = senum::SE_NEGATIVE as i32,
    // case SE_POSITIVE:
    //   i32 pos;
    SE_POSITIVE { pos: i32 } = senum::SE_POSITIVE as i32,
    // default:
    //   void;
    Default { d: i32 },
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
pub enum sunion_no_void {
    // case SE_NEGATIVE:
    //   bool neg;
    SE_NEGATIVE { neg: bool } = senum::SE_NEGATIVE as i32,
    // case SE_POSITIVE:
    //   i32 pos;
    SE_POSITIVE { pos: i32 } = senum::SE_POSITIVE as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
pub enum uunion {
    // case 1:
    //   bool one;
    _1 { one: bool } = 1 as i32,
    // case 2:
    //   i32 two;
    _2 { two: i32 } = 2 as i32,
    // case 4:
    //   BytesM four;
    _4 { four: BytesM } = 4 as i32,
    
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
pub enum uunion_bool {
    // case TRUE:
    //   bool one;
    TRUE { one: bool } = true as i32,
    // case FALSE:
    //   bool two;
    FALSE { two: bool } = false as i32,
    
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct test_recursive {
    pub elem: BytesM,
    // #[rust(box_ptr)]
    pub next: Option<Box<test_recursive>>,
    pub nextvec: VecM<test_recursive>,
}

// 
pub const kBytestringMaxLen : u32 = 1024;

pub type Bytestring = BytesM::<kBytestringMaxLen>;

pub type SHA256Hash = [u8; 32];

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct NonnegativeInt64SetAddArgs {
    // int64_t
    pub set_value: i64,
    // int64_t
    pub add_value: i64,
}


#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum StorageDeltaType {
    kDeleteLast = 0,
    kBytestringSet = 1,
    kNonnegativeInt64SetAdd = 2,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
pub enum StorageDelta {
    // case kDeleteLast:
    //   void;
    kDeleteLast = StorageDeltaType::kDeleteLast as i32,
    // case kBytestringSet:
    //   Bytestring bstring;
    kBytestringSet { bstring: Bytestring } = StorageDeltaType::kBytestringSet as i32,
    // case kNonnegativeInt64SetAdd:
    //   NonnegativeInt64SetAddArgs nnint64;
    kNonnegativeInt64SetAdd { nnint64: NonnegativeInt64SetAddArgs } = StorageDeltaType::kNonnegativeInt64SetAdd as i32,
    
}

pub type StorageDeltaVec = VecM::<StorageDelta>;


//
// XDR internals below this point
//

impl sunion {
    pub fn d(&self) -> i32 {
        match self {
            sunion::Default{ d } => *d ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl sunion_no_void {
    pub fn d(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl uunion {
    pub fn d(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl uunion_bool {
    pub fn d(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl StorageDelta {
    pub fn delta_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


