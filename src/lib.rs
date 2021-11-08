#[cfg(not(target_pointer_width = "64"))]
compile_error!("Pointer size must be 64 bits");

pub struct TaggedPointer<const BITS: u8> {
    ptr: usize,
}

impl<const BITS: u8> TaggedPointer<BITS> {
    const VALUE_BITS: u8 = 64 - BITS;

    fn new_from_usize<T, const MASK: usize>(value: usize) -> Self
    where
        T: TaggedPointerValue<BITS>,
    {
        debug_assert!(MASK < (1_usize << BITS));
        let tag_mask = MASK << Self::VALUE_BITS;
        let ptr = value | tag_mask;
        Self { ptr }
    }

    pub fn new<T, const MASK: usize>(value: T) -> Self
    where
        T: TaggedPointerValue<BITS> + std::fmt::Debug,
    {
        Self::new_from_usize::<T, MASK>(T::as_untagged_ptr(value))
    }

    pub fn is<const MASK: usize>(&self) -> bool {
        self.tag() == MASK
    }

    pub fn unwrap<T>(mut self) -> T
    where
        T: TaggedPointerValue<BITS>,
    {
        let untagged_ptr = self.without_tag();
        self.ptr = 0;
        T::unwrap(untagged_ptr)
    }

    pub fn borrow_value<T, U>(&self) -> &U
    where
        T: TaggedPointerValue<BITS> + std::borrow::Borrow<U>,
    {
        T::borrow_value(self)
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
}

pub trait TaggedPointerValue<const BITS: u8> {
    fn as_untagged_ptr(this: Self) -> usize
    where
        Self: Sized + std::fmt::Debug,
    {
        let mut untagged_ptr = 0;
        let dst: *mut u8 = &mut untagged_ptr as *mut usize as *mut u8;
        let src: *const u8 = &this as *const Self as *const u8;
        unsafe { std::ptr::copy(src, dst, std::mem::size_of::<Self>()) }
        std::mem::forget(this);
        untagged_ptr
    }
    fn from_untagged_ptr(untagged_ptr: usize) -> Self
    where
        Self: Sized,
    {
        let mut this = std::mem::MaybeUninit::<Self>::uninit();
        let dst = this.as_mut_ptr() as *mut u8;
        let src = &untagged_ptr as *const usize as *const u8;
        unsafe { std::ptr::copy(src, dst, std::mem::size_of::<Self>()) };
        unsafe { this.assume_init() }
    }

    fn unwrap(untagged_ptr: usize) -> Self
    where
        Self: Sized,
    {
        Self::from_untagged_ptr(untagged_ptr)
    }

    fn drop_inner(untagged_ptr: usize)
    where
        Self: Sized,
    {
        drop(Self::unwrap(untagged_ptr))
    }

    fn borrow_value<U>(tagged_ptr: &TaggedPointer<BITS>) -> &U
    where
        Self: std::borrow::Borrow<U> + Sized,
    {
        let untagged_ptr = tagged_ptr.without_tag();
        let this: Self = unsafe { std::mem::transmute_copy(&untagged_ptr) };
        let borrowed: &U = unsafe { std::mem::transmute(this.borrow()) };
        std::mem::forget(this);
        borrowed
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl<const BITS: u8> TaggedPointerValue<BITS> for bool {}

    impl<const BITS: u8> TaggedPointerValue<BITS> for u8 {}

    impl<T, const BITS: u8> TaggedPointerValue<BITS> for Box<T> {}

    const TEST_BITS: u8 = 5;
    const BOOL_TAG: usize = 1;
    const U8_TAG: usize = 2;
    const BOX_STRING_TAG: usize = 3;

    #[test]
    fn test_bool() {
        let ptr_true = TaggedPointer::<TEST_BITS>::new::<bool, BOOL_TAG>(true);
        assert_eq!(ptr_true.tag(), BOOL_TAG);
        assert_eq!(ptr_true.without_tag(), 1);
        assert!(ptr_true.is::<BOOL_TAG>());
        assert!(!ptr_true.is::<U8_TAG>());
        assert!(!ptr_true.is::<BOX_STRING_TAG>());
        assert_eq!(ptr_true.unwrap::<bool>(), true);

        let ptr_false = TaggedPointer::<TEST_BITS>::new::<bool, BOOL_TAG>(false);
        assert_eq!(ptr_false.tag(), BOOL_TAG);
        assert_eq!(ptr_false.without_tag(), 0);
        assert!(ptr_false.is::<BOOL_TAG>());
        assert!(!ptr_false.is::<U8_TAG>());
        assert!(!ptr_false.is::<BOX_STRING_TAG>());
        assert_eq!(ptr_false.unwrap::<bool>(), false);
    }

    #[test]
    fn test_u8() {
        let ptr42 = TaggedPointer::<TEST_BITS>::new::<u8, U8_TAG>(42);
        assert_eq!(ptr42.tag(), U8_TAG);
        assert_eq!(ptr42.without_tag(), 42);
        assert!(ptr42.is::<U8_TAG>());
        assert!(!ptr42.is::<BOOL_TAG>());
        assert!(!ptr42.is::<BOX_STRING_TAG>());
        assert_eq!(ptr42.unwrap::<u8>(), 42);
    }

    #[test]
    fn test_box() {
        let ptr = Box::new(String::from("foo"));
        let ptr_as_usize: usize = unsafe { std::mem::transmute_copy(&ptr) };
        let ptr_s = TaggedPointer::<TEST_BITS>::new::<Box<String>, BOX_STRING_TAG>(ptr);
        assert_eq!(ptr_s.tag(), BOX_STRING_TAG);
        assert_eq!(ptr_s.without_tag(), ptr_as_usize);
        assert!(ptr_s.is::<BOX_STRING_TAG>());
        assert!(!ptr_s.is::<BOOL_TAG>());
        assert!(!ptr_s.is::<U8_TAG>());
        assert_eq!(
            ptr_s.borrow_value::<Box<String>, String>(),
            &String::from("foo")
        );
        assert_eq!(ptr_s.unwrap::<Box<String>>(), Box::new(String::from("foo")));
    }

    #[test]
    fn test_drop() {
        let ptr = TaggedPointer::<TEST_BITS>::new::<bool, BOOL_TAG>(true);
        drop(ptr);

        let ptr = TaggedPointer::<TEST_BITS>::new::<u8, U8_TAG>(42);
        drop(ptr);

        let ptr = TaggedPointer::<TEST_BITS>::new::<Box<String>, BOX_STRING_TAG>(Box::new(
            String::from("foo"),
        ));
        drop(ptr);
    }
}
