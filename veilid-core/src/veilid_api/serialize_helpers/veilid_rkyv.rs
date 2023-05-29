use super::*;
use rkyv::ser::Serializer;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct VeilidRkyvSerializer<S> {
    inner: S,
}

impl<S> VeilidRkyvSerializer<S> {
    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S: rkyv::Fallible> rkyv::Fallible for VeilidRkyvSerializer<S> {
    type Error = VeilidRkyvError<S::Error>;
}

impl<S: rkyv::ser::ScratchSpace> rkyv::ser::ScratchSpace for VeilidRkyvSerializer<S> {
    unsafe fn push_scratch(
        &mut self,
        layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, Self::Error> {
        self.inner
            .push_scratch(layout)
            .map_err(VeilidRkyvError::Inner)
    }
    unsafe fn pop_scratch(
        &mut self,
        ptr: core::ptr::NonNull<u8>,
        layout: core::alloc::Layout,
    ) -> Result<(), Self::Error> {
        self.inner
            .pop_scratch(ptr, layout)
            .map_err(VeilidRkyvError::Inner)
    }
}

impl<S: rkyv::ser::Serializer> rkyv::ser::Serializer for VeilidRkyvSerializer<S> {
    #[inline]
    fn pos(&self) -> usize {
        self.inner.pos()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.write(bytes).map_err(VeilidRkyvError::Inner)
    }
}

impl<S: Default> Default for VeilidRkyvSerializer<S> {
    fn default() -> Self {
        Self {
            inner: S::default(),
        }
    }
}

pub type DefaultVeilidRkyvSerializer =
    VeilidRkyvSerializer<rkyv::ser::serializers::AllocSerializer<1024>>;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct VeilidSharedDeserializeMap {
    inner: SharedDeserializeMap,
}

impl VeilidSharedDeserializeMap {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: SharedDeserializeMap::new(),
        }
    }
}
impl rkyv::Fallible for VeilidSharedDeserializeMap {
    type Error = VeilidRkyvError<rkyv::de::deserializers::SharedDeserializeMapError>;
}

impl rkyv::de::SharedDeserializeRegistry for VeilidSharedDeserializeMap {
    fn get_shared_ptr(&mut self, ptr: *const u8) -> Option<&dyn rkyv::de::SharedPointer> {
        self.inner.get_shared_ptr(ptr)
    }

    fn add_shared_ptr(
        &mut self,
        ptr: *const u8,
        shared: Box<dyn rkyv::de::SharedPointer>,
    ) -> Result<(), Self::Error> {
        self.inner
            .add_shared_ptr(ptr, shared)
            .map_err(VeilidRkyvError::Inner)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum VeilidRkyvError<E> {
    Inner(E),
    StringError(String),
}

impl<E: Debug> From<String> for VeilidRkyvError<E> {
    fn from(s: String) -> Self {
        Self::StringError(s)
    }
}

impl<E: Debug + fmt::Display> fmt::Display for VeilidRkyvError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VeilidRkyvError::Inner(e) => write!(f, "Inner: {}", e),
            VeilidRkyvError::StringError(s) => write!(f, "StringError: {}", s),
        }
    }
}

impl<E: Debug + fmt::Display> std::error::Error for VeilidRkyvError<E> {}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn to_rkyv<T>(value: &T) -> VeilidAPIResult<Vec<u8>>
where
    T: RkyvSerialize<DefaultVeilidRkyvSerializer>,
{
    let mut serializer = DefaultVeilidRkyvSerializer::default();
    serializer
        .serialize_value(value)
        .map_err(|e| VeilidAPIError::generic(format!("failed to serialize object: {}", e)))?;
    Ok(serializer
        .into_inner()
        .into_serializer()
        .into_inner()
        .to_vec())
}

pub fn from_rkyv<T>(bytes: Vec<u8>) -> VeilidAPIResult<T>
where
    T: RkyvArchive,
    <T as RkyvArchive>::Archived:
        for<'t> CheckBytes<rkyv::validation::validators::DefaultValidator<'t>>,
    <T as RkyvArchive>::Archived: RkyvDeserialize<T, VeilidSharedDeserializeMap>,
{
    rkyv::check_archived_root::<T>(&bytes)
        .map_err(|e| VeilidAPIError::generic(format!("checkbytes failed: {}", e)))?
        .deserialize(&mut VeilidSharedDeserializeMap::default())
        .map_err(|e| VeilidAPIError::generic(format!("failed to deserialize: {}", e)))
}
