/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WhitelistMe<T> {
    pub foo: ::std::os::raw::c_int,
    pub bar: WhitelistMe_Inner<T>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WhitelistMe_Inner<T> {
    pub bar: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for WhitelistMe_Inner<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<T> Default for WhitelistMe<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
