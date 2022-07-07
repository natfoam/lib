use std::sync::atomic::AtomicU32;

type HRESULT = u32;

type GUID = u128;

type ULONG = u32;

struct IUnknown(&'static IUknownVmt);

struct IUknownVmt {
    QueryInterface: extern "stdcall" fn (this: &mut IUnknown, riid: &GUID, ppvObject: &mut &mut IUnknown) -> HRESULT,
    AddRef: extern "stdcall" fn (this: &mut IUnknown) -> ULONG,
    Release: extern "stdcall" fn (this: &mut IUnknown) -> ULONG,
}

#[repr(transparent)]
struct Ref(*mut IUnknown);

impl Ref {
    fn raw(&self) -> &mut IUnknown { unsafe { &mut *self.0 } }
    fn new(raw: &mut IUnknown) -> Self {
        (raw.0.AddRef)(raw);
        Self(raw)
    }
}

impl Drop for Ref {
    fn drop(&mut self) {
        let raw = self.raw();
        (raw.0.Release)(raw);
    }
}

impl Clone for Ref {
    fn clone(&self) -> Self { Self::new(self.raw()) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
