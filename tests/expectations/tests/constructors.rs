/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct TestOverload {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_TestOverload() {
    assert_eq!(::std::mem::size_of::<TestOverload>() , 1usize , concat ! (
               "Size of: " , stringify ! ( TestOverload ) ));
    assert_eq! (::std::mem::align_of::<TestOverload>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( TestOverload ) ));
}
extern "C" {
    #[link_name = "_ZN12TestOverloadC1Ei"]
    pub fn TestOverload_TestOverload(this: *mut TestOverload,
                                     arg1: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "_ZN12TestOverloadC1Ed"]
    pub fn TestOverload_TestOverload1(this: *mut TestOverload, arg1: f64);
}
impl Clone for TestOverload {
    fn clone(&self) -> Self { *self }
}
impl TestOverload {
    #[inline]
    pub unsafe fn new(arg1: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        TestOverload_TestOverload(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
    #[inline]
    pub unsafe fn new1(arg1: f64) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        TestOverload_TestOverload1(&mut __bindgen_tmp, arg1);
        __bindgen_tmp
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct TestPublicNoArgs {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_TestPublicNoArgs() {
    assert_eq!(::std::mem::size_of::<TestPublicNoArgs>() , 1usize , concat ! (
               "Size of: " , stringify ! ( TestPublicNoArgs ) ));
    assert_eq! (::std::mem::align_of::<TestPublicNoArgs>() , 1usize , concat !
                ( "Alignment of " , stringify ! ( TestPublicNoArgs ) ));
}
extern "C" {
    #[link_name = "_ZN16TestPublicNoArgsC1Ev"]
    pub fn TestPublicNoArgs_TestPublicNoArgs(this: *mut TestPublicNoArgs);
}
impl Clone for TestPublicNoArgs {
    fn clone(&self) -> Self { *self }
}
impl TestPublicNoArgs {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        TestPublicNoArgs_TestPublicNoArgs(&mut __bindgen_tmp);
        __bindgen_tmp
    }
}
