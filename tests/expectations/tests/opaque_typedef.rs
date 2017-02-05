/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RandomTemplate<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
impl <T> Default for RandomTemplate<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
/** <div rustbindgen opaque></div> */
pub type ShouldBeOpaque = [u8; 0usize];
pub type ShouldNotBeOpaque = RandomTemplate<f32>;
