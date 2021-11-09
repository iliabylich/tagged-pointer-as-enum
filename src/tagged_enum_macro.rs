#[macro_export]
macro_rules! tagged_enum {
    (
        enum $enum:ident {
            $name1:ident($t1:ty),
            $name2:ident($t2:ty),
            bits = $bits:literal
        }
    ) => {
        #[repr(transparent)]
        pub struct $enum {
            pointer: $crate::TaggedPointer<$bits>,
        }

        #[allow(non_upper_case_globals)]
        mod tags {
            pub(crate) const $name1: usize = 1;
            pub(crate) const $name2: usize = 2;
        }

        impl $enum {
            #[allow(non_snake_case)]
            pub(crate) fn $name1(value: $t1) -> Self {
                Self {
                    pointer: $crate::TaggedPointer::new(value, tags::$name1),
                }
            }

            #[allow(non_snake_case)]
            pub(crate) fn $name2(value: $t2) -> Self {
                Self {
                    pointer: $crate::TaggedPointer::new(value, tags::$name2),
                }
            }

            pub(crate) fn is(&self, tag: usize) -> bool {
                self.pointer.tag() == tag
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
            U8(u8),
            StringPtr(Box<String>),
            bits = 8
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
    fn test_u8() {
        let u8_ptr = TestEnum::U8(42);
        assert!(u8_ptr.is(tags::U8));
        assert!(!u8_ptr.is(tags::StringPtr));
        assert_eq!(u8_ptr.unwrap::<u8>(), 42);
    }

    #[test]
    fn test_string_ptr() {
        let u16_ptr = TestEnum::StringPtr(Box::new(String::from("foo")));
        assert!(!u16_ptr.is(tags::U8));
        assert!(u16_ptr.is(tags::StringPtr));
        assert_eq!(u16_ptr.unwrap::<StringPtr>().as_ref(), &String::from("foo"));
    }
}
