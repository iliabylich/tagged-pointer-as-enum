macro_rules! __declare_tags {
    ( start = $n:expr; ) => {};
    ( start = $n:expr; $x:ident $(, $name:ident)* ) => {
        pub(crate) const $x: usize = $n;
        __declare_tags!(start = $n + 1; $($name),* );
    };
}

#[test]
fn test_declare_tags() {
    __declare_tags!(start = 42; A, B, C);
    assert_eq!(A, 42);
    assert_eq!(B, 43);
    assert_eq!(C, 44);
}

#[macro_export]
macro_rules! tagged_enum {
    (
        enum $enum:ident {
            bits = $bits:literal;
            $($name:ident($t:ty),)+
        }
    ) => {
        #[repr(transparent)]
        pub struct $enum {
            pointer: $crate::TaggedPointer<$bits>,
        }

        #[allow(non_upper_case_globals)]
        mod tags {
            __declare_tags!(start = 1; $($name),+);
        }

        impl $enum {
            // constructors
            $(
                #[allow(non_snake_case)]
                pub(crate) fn $name(value: $t) -> Self {
                    Self {
                        pointer: $crate::TaggedPointer::new(value, tags::$name),
                    }
                }
            )+

            pub(crate) fn tag(&self) -> usize {
                self.pointer.tag()
            }

            pub(crate) fn is(&self, tag: usize) -> bool {
                self.tag() == tag
            }

            pub(crate) fn unwrap<U>(self) -> U
            where
                U: $crate::TaggedPointerValue,
            {
                self.pointer.unwrap::<U>()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    type StringPtr = Box<String>;

    tagged_enum! {
        enum TestEnum {
            bits = 8;

            U8(u8),
            StringPtr(StringPtr),
        }
    }

    #[test]
    fn test_size() {
        assert_eq!(
            std::mem::size_of::<TestEnum>(),
            std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_tags() {
        assert_eq!(tags::U8, 1);
        assert_eq!(tags::StringPtr, 2);
    }

    #[test]
    fn test_u8() {
        let u8_ptr = TestEnum::U8(42);
        assert!(u8_ptr.is(tags::U8));
        assert!(!u8_ptr.is(tags::StringPtr));
        assert_eq!(u8_ptr.unwrap::<u8>(), 42);
    }

    #[test]
    fn test_string_ptr() {
        let string_ptr = TestEnum::StringPtr(Box::new(String::from("foo")));
        assert!(!string_ptr.is(tags::U8));
        assert!(string_ptr.is(tags::StringPtr));
        assert_eq!(
            string_ptr.unwrap::<StringPtr>().as_ref(),
            &String::from("foo")
        );
    }
}
