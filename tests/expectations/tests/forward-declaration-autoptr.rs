/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RefPtr<T> {
    pub m_inner: *mut T,
}
impl <T> Default for RefPtr<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Bar {
    pub m_member: RefPtr<Foo>,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 8usize , concat ! (
               "Size of: " , stringify ! ( Bar ) ));
    assert_eq! (::std::mem::align_of::<Bar>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( Bar ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Bar ) ) . m_member as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( Bar ) , "::" ,
                stringify ! ( m_member ) ));
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}
impl Default for Bar {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
