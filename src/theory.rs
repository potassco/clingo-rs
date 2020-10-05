#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::{Control, Model, Statistics, Symbol};
use clingo_sys::*;
use std::ffi::CStr;
use std::os::raw::c_void;
/// Object to add command-line options.
pub struct Options(clingo_options_t);
pub trait Theory {
    /// creates the theory
    fn create() -> Self;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_create<T: Theory>(theory: *mut *mut c_void) -> bool {
        let theorybox = Box::<T>::new(T::create());
        let theory_ptr = Box::<T>::into_raw(theorybox);
        if theory_ptr.is_null() {
            false
        } else {
            *theory = theory_ptr as *mut c_void;
            true
        }
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_destroy<T: Theory>(theory: *mut c_void) -> bool {
        // Converting the raw pointer back into a Box  for automatic cleanup
        let _theorybox = Box::from_raw(theory);
        true
    }
    /// registers the theory with the control
    fn register(&mut self, ctl: &mut Control) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_register<T: Theory>(
        theory: *mut c_void,
        control: *mut clingo_control_t,
    ) -> bool {
        let control = &mut *(control as *mut Control);
        let theory = &mut *(theory as *mut T);
        theory.register(control)
    }
    /// prepare the theory between grounding and solving
    fn prepare(&mut self, ctl: &mut Control) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_prepare<T: Theory>(
        theory: *mut c_void,
        control: *mut clingo_control_t,
    ) -> bool {
        let control = &mut *(control as *mut Control);
        let theory = &mut *(theory as *mut T);
        theory.prepare(control)
    }
    /// add options for your theory
    fn register_options(&mut self, options: &mut Options) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_register_options<T: Theory>(
        theory: *mut c_void,
        options: *mut clingo_options_t,
    ) -> bool {
        let options = &mut *(options as *mut Options);
        let theory = &mut *(theory as *mut T);
        theory.register_options(options)
    }
    /// validate options for your theory
    fn validate_options(&mut self) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_validate_options<T: Theory>(theory: *mut c_void) -> bool {
        let theory = &mut *(theory as *mut T);
        theory.validate_options()
    }
    /// callback on every model
    fn on_model(&mut self, model: &mut Model) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_on_model<T: Theory>(
        theory: *mut c_void,
        model: *mut clingo_model_t,
    ) -> bool {
        let model = &mut *(model as *mut Model);
        let theory = &mut *(theory as *mut T);
        theory.on_model(model)
    }
    /// callback on statistic updates
    /// please add a subkey with the name of your theory
    fn on_statistics(&mut self, step: &mut Statistics, akku: &mut Statistics) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_on_statistics<T: Theory>(
        theory: *mut c_void,
        step: *mut clingo_statistics_t,
        accu: *mut clingo_statistics_t,
    ) -> bool {
        let step = &mut *(step as *mut Statistics);
        let accu = &mut *(accu as *mut Statistics);
        let theory = &mut *(theory as *mut T);
        theory.on_statistics(step, accu)
    }
    /// obtain a symbol index which can be used to get the value of a symbol
    /// returns true if the symbol exists
    /// does not throw
    fn lookup_symbol(&mut self, symbol: Symbol, index: &mut usize) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_lookup_symbol<T: Theory>(
        theory: *mut c_void,
        symbol: clingo_symbol_t,
        index: *mut usize,
    ) -> bool {
        let index = &mut *(index as *mut usize);
        let theory = &mut *(theory as *mut T);
        theory.lookup_symbol(Symbol(symbol), index)
    }
    /// obtain the symbol at the given index
    /// does not throw
    fn get_symbol(&mut self, index: usize) -> Symbol;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_get_symbol<T: Theory>(
        theory: *mut c_void,
        index: usize,
    ) -> clingo_symbol_t {
        let theory = &mut *(theory as *mut T);
        let symbol = theory.get_symbol(index);
        symbol.0
    }
    /// initialize index so that it can be used with clingodl_assignment_next
    /// does not throw
    fn assignment_begin(&mut self, thread_id: u32, index: &mut usize);
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_assignment_begin<T: Theory>(
        theory: *mut c_void,
        thread_id: u32,
        index: *mut usize,
    ) {
        let index = &mut *(index as *mut usize);
        let theory = &mut *(theory as *mut T);
        theory.assignment_begin(thread_id, index);
    }
    /// move to the next index that has a value
    /// returns true if the updated index is valid
    /// does not throw
    fn assignment_next(&mut self, thread_id: u32, index: &mut usize) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_assignment_next<T: Theory>(
        theory: *mut c_void,
        thread_id: u32,
        index: *mut usize,
    ) -> bool {
        let index = &mut *(index as *mut usize);
        let theory = &mut *(theory as *mut T);
        theory.assignment_next(thread_id, index)
    }
    /// check if the symbol at the given index has a value
    /// does not throw
    fn assignment_has_value(&mut self, thread_id: u32, index: usize) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_assignment_has_value<T: Theory>(
        theory: *mut c_void,
        thread_id: u32,
        index: usize,
    ) -> bool {
        let theory = &mut *(theory as *mut T);
        theory.assignment_has_value(thread_id, index)
    }
    /// get the symbol and it's value at the given index
    /// does not throw
    fn assignment_get_value(&mut self, thread_id: u32, index: usize, value: &mut TheoryValue);
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_assignment_get_value<T: Theory>(
        theory: *mut c_void,
        thread_id: u32,
        index: usize,
        value: *mut value_t,
    ) {
        let value = &mut *(value as *mut TheoryValue);
        let theory = &mut *(theory as *mut T);
        theory.assignment_get_value(thread_id, index, value)
    }
    /// configure theory manually (without using clingo's options facility)
    /// Note that the theory has to be configured before registering it and cannot be reconfigured.
    fn configure(&mut self, key: &str, value: &str) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_configure<T: Theory>(
        theory: *mut c_void,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> bool {
        let key_cstr = CStr::from_ptr(key);
        let value_cstr = CStr::from_ptr(value);
        let key = key_cstr.to_str().unwrap();
        let value = value_cstr.to_str().unwrap();
        // Ok(out_cstr.to_str()?)
        let theory = &mut *(theory as *mut T);
        theory.configure(key, value)
    }
}
pub type value_type_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct value {
    pub type_: value_type_t,
    pub __bindgen_anon_1: value__bindgen_ty_1,
}
pub struct TheoryValue(value_t);
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
