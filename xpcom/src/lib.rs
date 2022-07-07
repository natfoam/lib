type HRESULT = u32;

type GUID = u128;

type ULONG = u32;

//

pub trait Interface: 'static {
    const ID: GUID;
}

#[repr(transparent)]
pub struct Obj<I: Interface> (&'static Vmt<I>);

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

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }
}
