use std::{ptr::{null_mut}, sync::atomic::{AtomicU32, Ordering}};

type HRESULT = u32;

const S_OK: HRESULT = 0;
const E_NOINTERFACE: HRESULT = 0x80004002;
const E_POINTER: HRESULT = 0x80004003;

type GUID = u128;

type ULONG = u32;

//

pub trait Interface: 'static {
    const ID: GUID;
}

#[repr(transparent)]
pub struct Obj<I: Interface> (&'static Vmt<Obj<I>, I>);

#[allow(non_snake_case)]
#[repr(C)]
struct Vmt<T, I: Interface> {
    QueryInterface: extern "stdcall" fn (this: &mut T, riid: &GUID, ppvObject: *mut *mut T) -> HRESULT,
    AddRef: extern "stdcall" fn (this: &mut T) -> ULONG,
    Release: extern "stdcall" fn (this: &mut T) -> ULONG,
    interface: I
}

#[repr(transparent)]
pub struct Ref<I: Interface>(*mut Obj<I>);

impl<I: Interface> Ref<I> {
    pub fn raw(&self) -> &mut Obj<I> { unsafe { &mut *self.0 } }
    pub fn new(raw: &mut Obj<I>) -> Self {
        (raw.0.AddRef)(raw);
        Self(raw)
    }
}

impl<I: Interface> Drop for Ref<I> {
    fn drop(&mut self) {
        let raw = self.raw();
        (raw.0.Release)(raw);
    }
}

impl<I: Interface> Clone for Ref<I> {
    fn clone(&self) -> Self { Self::new(self.raw()) }
}

//

pub trait Class: 'static + Sized {
    type Interface: Interface;
    const INTERFACE: Self::Interface;
}

#[repr(C)]
struct ClassObj<C: Class> {
    vmt: &'static Vmt<Self, C::Interface>,
    counter: AtomicU32,
    value: C,
}

#[allow(non_snake_case)]
trait ClassEx: Class {
    const VMT: Vmt<ClassObj<Self>, Self::Interface> = Vmt {
        QueryInterface: Self::QueryInterface,
        AddRef: Self::AddRef,
        Release: Self::Release,
        interface: Self::INTERFACE,
    };
    extern "stdcall" fn QueryInterface(this: &mut ClassObj<Self>, riid: &GUID, ppvObject: *mut *mut ClassObj<Self>) -> HRESULT {
        if ppvObject == null_mut() {
            return E_POINTER
        }
        let result: (*mut ClassObj<Self>, HRESULT) = if Self::Interface::ID == *riid {
            Self::AddRef(this);
            (this, S_OK)
        } else {
            (null_mut(), E_NOINTERFACE)
        };
        unsafe { *ppvObject = result.0 }
        result.1
    }
    extern "stdcall" fn AddRef(this: &mut ClassObj<Self>) -> ULONG {
        this.counter.fetch_add(1, Ordering::Relaxed) + 1
    }
    extern "stdcall" fn Release(this: &mut ClassObj<Self>) -> ULONG {
        match this.counter.fetch_sub(1, Ordering::Relaxed) {
            0 => panic!("release"),
            1 => { 
                unsafe { Box::from_raw(this) };
                0
            }
            c => c - 1
        }
    }
} 

impl<C: Class> ClassEx for C {}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }
}
