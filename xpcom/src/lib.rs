type HRESULT = u32;

const S_OK: HRESULT = 0;

type GUID = u128;

type ULONG = u32;

//

pub trait Interface: 'static {
    const ID: GUID;
}

#[repr(transparent)]
pub struct Obj<I: Interface> (&'static Vmt<I>);

#[allow(non_snake_case)]
#[repr(C)]
struct Vmt<I: Interface> {
    QueryInterface: extern "stdcall" fn (this: &mut Obj<I>, riid: &GUID, ppvObject: &mut &mut Obj<I>) -> HRESULT,
    AddRef: extern "stdcall" fn (this: &mut Obj<I>) -> ULONG,
    Release: extern "stdcall" fn (this: &mut Obj<I>) -> ULONG,
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

pub trait Class {
    type Interface: Interface;
    const INTERFACE: Self::Interface;
}

#[allow(non_snake_case)]
trait ClassEx: Class {
    const VMT: Vmt<Self::Interface> = Vmt {
        QueryInterface: Self::QueryInterface,
        AddRef: Self::AddRef,
        Release: Self::Release,
        interface: Self::INTERFACE,
    };
    extern "stdcall" fn QueryInterface(_this: &mut Obj<Self::Interface>, _riid: &GUID, _ppvObject: &mut &mut Obj<Self::Interface>) -> HRESULT {
        S_OK
    }
    extern "stdcall" fn AddRef(_this: &mut Obj<Self::Interface>) -> ULONG { 0 }
    extern "stdcall" fn Release(_this: &mut Obj<Self::Interface>) -> ULONG { 0 }
} 

impl<C: Class> ClassEx for C {}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }
}
