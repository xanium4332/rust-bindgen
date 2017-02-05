/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


/**
 * <div rustbindgen opaque></div>
 */
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct OtherOpaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_OtherOpaque() {
    assert_eq!(::std::mem::size_of::<OtherOpaque>() , 4usize , concat ! (
               "Size of: " , stringify ! ( OtherOpaque ) ));
    assert_eq! (::std::mem::align_of::<OtherOpaque>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( OtherOpaque ) ));
}
impl Clone for OtherOpaque {
    fn clone(&self) -> Self { *self }
}
/**
 * <div rustbindgen opaque></div>
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Opaque<T> {
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
impl <T> Default for Opaque<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct WithOpaquePtr {
    pub whatever: *mut Opaque<::std::os::raw::c_int>,
    pub other: u32,
    pub t: OtherOpaque,
}
#[test]
fn bindgen_test_layout_WithOpaquePtr() {
    assert_eq!(::std::mem::size_of::<WithOpaquePtr>() , 16usize , concat ! (
               "Size of: " , stringify ! ( WithOpaquePtr ) ));
    assert_eq! (::std::mem::align_of::<WithOpaquePtr>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( WithOpaquePtr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const WithOpaquePtr ) ) . whatever as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( WithOpaquePtr ) , "::"
                , stringify ! ( whatever ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const WithOpaquePtr ) ) . other as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( WithOpaquePtr ) , "::"
                , stringify ! ( other ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const WithOpaquePtr ) ) . t as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( WithOpaquePtr ) , "::"
                , stringify ! ( t ) ));
}
impl Clone for WithOpaquePtr {
    fn clone(&self) -> Self { *self }
}
impl Default for WithOpaquePtr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
