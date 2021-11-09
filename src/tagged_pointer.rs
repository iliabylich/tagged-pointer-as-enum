use crate::TaggedPointerValue;

pub struct TaggedPointer<const BITS: u8> {
    ptr: usize,
}

impl<const BITS: u8> TaggedPointer<BITS> {
    const VALUE_BITS: u8 = 64 - BITS;

    fn new_from_usize<T>(value: usize, tag: usize) -> Self
    where
        T: TaggedPointerValue,
    {
        debug_assert!(tag < (1_usize << BITS));
        let tag_mask = tag << Self::VALUE_BITS;
        let ptr = value | tag_mask;
        Self { ptr }
    }

    pub fn new<T>(value: T, tag: usize) -> Self
    where
        T: TaggedPointerValue,
    {
        Self::new_from_usize::<T>(T::as_untagged_ptr(value), tag)
    }

    pub fn is(&self, tag: usize) -> bool {
        self.tag() == tag
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

    pub fn take(&mut self) -> Self {
        let taken = Self { ptr: self.ptr };
        self.ptr = 0;
        taken
    }

    pub fn format_as<T>(
        &self,
        variant_name: &str,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    where
        T: TaggedPointerValue + std::fmt::Debug,
    {
        let copy = Self { ptr: self.ptr };
        let unwrapped = copy.unwrap::<T>();
        let fmt_result = write!(f, "{}({:?})", variant_name, unwrapped);
        std::mem::forget(unwrapped);
        fmt_result
    }

    pub fn clone_as<T>(&self) -> Self
    where
        T: TaggedPointerValue + Clone,
    {
        let copy = Self { ptr: self.ptr };
        let unwrapped = copy.unwrap::<T>();
        let clone = Self::new::<T>(unwrapped.clone(), self.tag());
        std::mem::forget(unwrapped);
        clone
    }

    pub fn compare_as<T>(&self, other: &Self) -> bool
    where
        T: TaggedPointerValue + PartialEq,
    {
        let l_copy = Self { ptr: self.ptr };
        let l_unwrapped = l_copy.unwrap::<T>();

        let r_copy = Self { ptr: other.ptr };
        let r_unwrapped = r_copy.unwrap::<T>();

        let cmp = l_unwrapped == r_unwrapped;

        std::mem::forget(l_unwrapped);
        std::mem::forget(r_unwrapped);

        cmp
    }
}
