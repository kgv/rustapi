use self::get_parent::{GetParent, GetParentBuilder};
use winapi::{
    shared::dxgi::{IDXGIDeviceSubObject, IDXGIObject, IDXGISwapChain},
    Interface,
};
use wio::com::ComPtr;

// IDXGIObject::GetParent	Gets the parent of the object.
// IDXGIObject::GetPrivateData	Get a pointer to the object's data.
// IDXGIObject::SetPrivateData	Sets application-defined data to the object and associates that data with a GUID.
// IDXGIObject::SetPrivateDataInterface	Set an interface in the object's private data.

/// Object.
pub trait Object {
    fn parent<T: Interface>(&self) -> GetParentBuilder<((ComPtr<IDXGIObject>,),), T>;
}

impl Object for ComPtr<IDXGIObject> {
    fn parent<T: Interface>(&self) -> GetParentBuilder<((ComPtr<IDXGIObject>,),), T> {
        GetParent::builder().object(self.clone())
    }
}

impl Object for ComPtr<IDXGIDeviceSubObject> {
    fn parent<T: Interface>(&self) -> GetParentBuilder<((ComPtr<IDXGIObject>,),), T> {
        GetParent::builder().object(self.cast().unwrap())
    }
}

impl Object for ComPtr<IDXGISwapChain> {
    fn parent<T: Interface>(&self) -> GetParentBuilder<((ComPtr<IDXGIObject>,),), T> {
        GetParent::builder().object(self.cast().unwrap())
    }
}

mod get_parent;
