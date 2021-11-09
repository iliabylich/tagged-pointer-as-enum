use crate::TaggedPointerValue;

pub struct TaggedPointer<const BITS: u8> {
    ptr: usize,
}

impl<const BITS: u8> TaggedPointer<BITS> {
    const VALUE_BITS: u8 = 64 - BITS;

    fn new_from_usize<T>(value: usize, mask: usize) -> Self
    where
        T: TaggedPointerValue,
    {
        debug_assert!(mask < (1_usize << BITS));
        let tag_mask = mask << Self::VALUE_BITS;
        let ptr = value | tag_mask;
        Self { ptr }
    }

    pub fn new<T>(value: T, mask: usize) -> Self
    where
        T: TaggedPointerValue,
    {
        Self::new_from_usize::<T>(T::as_untagged_ptr(value), mask)
    }

    pub fn is(&self, mask: usize) -> bool {
        self.tag() == mask
    }

    pub fn unwrap<T>(mut self) -> T
    where
        T: TaggedPointerValue,
    {
        let untagged_ptr = self.without_tag();
        self.ptr = 0;
        T::unwrap(untagged_ptr)
    }

    pub fn borrow_value<T, U>(&self) -> &U
    where
        T: TaggedPointerValue + std::borrow::Borrow<U>,
    {
        T::borrow_value::<U, BITS>(self)
    }

    pub fn tag(&self) -> usize {
        self.ptr >> Self::VALUE_BITS
    }

    fn tag_mask(&self) -> usize {
        self.tag() << Self::VALUE_BITS
    }

    pub fn without_tag(&self) -> usize {
        self.ptr ^ self.tag_mask()
    }

    pub fn drop_as<T>(&mut self)
    where
        T: TaggedPointerValue,
    {
        let untagged_ptr = self.without_tag();
        self.ptr = 0;
        drop(T::unwrap(untagged_ptr));
    }
}
