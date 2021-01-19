#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::{Control, Model, Options, Statistics, Symbol};
use super::ast;
use clingo_sys::*;
pub trait Theory {
    /// registers the theory with the control
    fn register(&mut self, ctl: &mut Control) -> bool;
    /// Rewrite statements before adding them via the given callback.
    fn rewrite_statement(&mut self, stm: &ast::Statement, builder: &mut ast::ProgramBuilder)-> bool;
    /// prepare the theory between grounding and solving
    fn prepare(&mut self, ctl: &mut Control) -> bool;
    /// add options for your theory
    fn register_options(&mut self, options: &mut Options) -> bool;
    /// validate options for your theory
    fn validate_options(&mut self) -> bool;
    /// callback on every model
    fn on_model(&mut self, model: &mut Model) -> bool;
    /// callback on statistic updates
    /// please add a subkey with the name of your theory
    fn on_statistics(&mut self, step: &mut Statistics, akku: &mut Statistics) -> bool;
    /// obtain a symbol index which can be used to get the value of a symbol
    /// returns true if the symbol exists
    /// does not throw
    fn lookup_symbol(&mut self, symbol: Symbol, index: &mut usize) -> bool;    /// obtain the symbol at the given index
    /// does not throw
    fn get_symbol(&mut self, index: usize) -> Symbol;
    /// initialize index so that it can be used with clingodl_assignment_next
    /// does not throw
    fn assignment_begin(&mut self, thread_id: u32, index: &mut usize);
    /// move to the next index that has a value
    /// returns true if the updated index is valid
    /// does not throw
    fn assignment_next(&mut self, thread_id: u32, index: &mut usize) -> bool;
    /// check if the symbol at the given index has a value
    /// does not throw
    fn assignment_has_value(&mut self, thread_id: u32, index: usize) -> bool;
    /// get the symbol and it's value at the given index
    /// does not throw
    fn assignment_get_value(&mut self, thread_id: u32, index: usize) -> TheoryValue;
    /// configure theory manually (without using clingo's options facility)
    /// Note that the theory has to be configured before registering it and cannot be reconfigured.
    fn configure(&mut self, key: &str, value: &str) -> bool;
}
pub type value_type_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct value {
    pub type_: value_type_t,
    pub __bindgen_anon_1: value__bindgen_ty_1,
}
// TODO: make TheoryValue an enum
pub struct TheoryValue(pub value_t);
#[repr(C)]
#[derive(Copy, Clone)]
pub union value__bindgen_ty_1 {
    pub int_number: ::std::os::raw::c_int,
    pub double_number: f64,
    pub symbol: clingo_symbol_t,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_value__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<value__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(value__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<value__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(value__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value__bindgen_ty_1>())).int_number as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value__bindgen_ty_1),
            "::",
            stringify!(int_number)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<value__bindgen_ty_1>())).double_number as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value__bindgen_ty_1),
            "::",
            stringify!(double_number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value__bindgen_ty_1>())).symbol as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value__bindgen_ty_1),
            "::",
            stringify!(symbol)
        )
    );
}
#[test]
fn bindgen_test_layout_value() {
    assert_eq!(
        ::std::mem::size_of::<value>(),
        16usize,
        concat!("Size of: ", stringify!(value))
    );
    assert_eq!(
        ::std::mem::align_of::<value>(),
        8usize,
        concat!("Alignment of ", stringify!(value))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<value>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(value),
            "::",
            stringify!(type_)
        )
    );
}
pub type value_t = value;
