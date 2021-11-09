#![doc = include_str!("../README.md")]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Pointer size must be 64 bits");

mod tagged_pointer;
pub use tagged_pointer::TaggedPointer;

mod tagged_pointer_value;
pub use tagged_pointer_value::TaggedPointerValue;

#[macro_use]
mod macros;

impl TaggedPointerValue for bool {}

impl TaggedPointerValue for u8 {}
impl TaggedPointerValue for u16 {}
impl TaggedPointerValue for u32 {}
impl TaggedPointerValue for i8 {}
impl TaggedPointerValue for i16 {}
impl TaggedPointerValue for i32 {}

impl<T> TaggedPointerValue for Box<T> {}
impl<T> TaggedPointerValue for Option<Box<T>> {}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_BITS: u8 = 5;
    const BOOL_TAG: usize = 1;
    const U8_TAG: usize = 2;
    const BOX_STRING_TAG: usize = 3;
    const OPTION_BOX_STRING_TAG: usize = 4;

    #[test]
    fn test_bool() {
        let ptr_true = TaggedPointer::<TEST_BITS>::new::<bool, BOOL_TAG>(true);
        assert_eq!(ptr_true.tag(), BOOL_TAG);
        assert!(ptr_true.is::<BOOL_TAG>());
        assert!(!ptr_true.is::<U8_TAG>());
        assert_eq!(ptr_true.unwrap::<bool>(), true);

        let ptr_false = TaggedPointer::<TEST_BITS>::new::<bool, BOOL_TAG>(false);
        assert_eq!(ptr_false.tag(), BOOL_TAG);
        assert!(ptr_false.is::<BOOL_TAG>());
        assert!(!ptr_false.is::<U8_TAG>());
        assert_eq!(ptr_false.unwrap::<bool>(), false);
    }

    #[test]
    fn test_u8() {
        let ptr42 = TaggedPointer::<TEST_BITS>::new::<u8, U8_TAG>(42);
        assert_eq!(ptr42.tag(), U8_TAG);
        assert!(ptr42.is::<U8_TAG>());
        assert!(!ptr42.is::<BOOL_TAG>());
        assert_eq!(ptr42.unwrap::<u8>(), 42);
    }

    #[test]
    fn test_box() {
        let ptr = Box::new(String::from("foo"));
        let ptr_s = TaggedPointer::<TEST_BITS>::new::<Box<String>, BOX_STRING_TAG>(ptr);
        assert_eq!(ptr_s.tag(), BOX_STRING_TAG);
        assert!(ptr_s.is::<BOX_STRING_TAG>());
        assert!(!ptr_s.is::<BOOL_TAG>());
        assert_eq!(
            ptr_s.borrow_value::<Box<String>, String>(),
            &String::from("foo")
        );
        assert_eq!(ptr_s.unwrap::<Box<String>>(), Box::new(String::from("foo")));
    }

    #[test]
    fn test_option_box() {
        let some_ptr = Some(Box::new(String::from("foo")));
        let ptr_s =
            TaggedPointer::<TEST_BITS>::new::<Option<Box<String>>, OPTION_BOX_STRING_TAG>(some_ptr);
        assert_eq!(ptr_s.tag(), OPTION_BOX_STRING_TAG);
        assert!(ptr_s.is::<OPTION_BOX_STRING_TAG>());
        assert!(!ptr_s.is::<BOOL_TAG>());
        assert_eq!(
            ptr_s.unwrap::<Option<Box<String>>>(),
            Some(Box::new(String::from("foo")))
        );
    }

    #[test]
    fn test_drop() {
        let mut ptr = TaggedPointer::<TEST_BITS>::new::<bool, BOOL_TAG>(true);
        ptr.drop_as::<bool>();

        let mut ptr = TaggedPointer::<TEST_BITS>::new::<u8, U8_TAG>(42);
        ptr.drop_as::<u8>();

        let mut ptr = TaggedPointer::<TEST_BITS>::new::<Box<String>, BOX_STRING_TAG>(Box::new(
            String::from("foo"),
        ));
        ptr.drop_as::<Box<String>>();
    }
}
