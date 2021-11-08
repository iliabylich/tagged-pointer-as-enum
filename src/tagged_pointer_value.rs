use crate::TaggedPointer;

pub trait TaggedPointerValue {
    fn as_untagged_ptr(this: Self) -> usize
    where
        Self: Sized,
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

    fn borrow_value<U, const BITS: u8>(tagged_ptr: &TaggedPointer<BITS>) -> &U
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
