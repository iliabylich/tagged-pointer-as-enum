#[macro_export]
macro_rules! tagged_enum {
    (
        $vis:vis enum $enum:ident {
            bits = $bits:literal;
            $($name:ident($t:ty),)+
        }
    ) => {
        #[repr(transparent)]
        $vis struct $enum {
            pointer: $crate::TaggedPointer<$bits>,
        }

        #[allow(non_upper_case_globals)]
        $vis mod tags {
            $crate::__declare_tags!(start = 1; $vis $($name),+);
        }

        impl $enum {
            // constructors
            $(
                #[allow(non_snake_case)]
                $vis fn $name(value: $t) -> Self {
                    Self {
                        pointer: $crate::TaggedPointer::new::<$t, {tags::$name}>(value),
                    }
                }
            )+

            $vis fn tag(&self) -> usize {
                self.pointer.tag()
            }

            $vis fn is<const TAG: usize>(&self) -> bool {
                self.pointer.is::<TAG>()
            }

            $vis fn borrow_value<T, U>(&self) -> &U
            where
                T: $crate::TaggedPointerValue + std::borrow::Borrow<U>,
            {
                self.pointer.borrow_value::<T, U>()
            }

            $vis fn unwrap<U>(mut self) -> U
            where
                U: $crate::TaggedPointerValue,
            {
                self.pointer.take().unwrap::<U>()
            }
        }

        $crate::__derive_macro!($enum; Drop; $($name : $t ),+ );
    };

    // version with extra derives
    (
        #[derive( $($d:ident),* )]
        $vis:vis enum $enum:ident {
            bits = $bits:literal;
            $($name:ident($t:ty),)+
        }
    ) => {
        $crate::tagged_enum! {
            $vis enum $enum {
                bits = $bits;
                $($name($t),)+
            }
        }

        $crate::__derive_macros!($enum; $($d,)* ; $($name : $t),+ );
    }
}

#[cfg(test)]
mod tests {
    type StringPtr = Box<String>;

    tagged_enum! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub(crate) enum TestEnum {
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

        assert!(u8_ptr.is::<{ tags::U8 }>());
        assert!(!u8_ptr.is::<{ tags::StringPtr }>());

        assert_eq!(format!("{:?}", u8_ptr), "U8(42)");

        let clone = u8_ptr.clone();
        assert_eq!(clone.unwrap::<u8>(), 42);

        assert_eq!(u8_ptr, u8_ptr.clone());
        assert_eq!(u8_ptr, TestEnum::U8(42));
        assert_ne!(u8_ptr, TestEnum::U8(43));
    }

    #[test]
    fn test_string_ptr() {
        let string_ptr = TestEnum::StringPtr(Box::new(String::from("foo")));

        assert!(!string_ptr.is::<{ tags::U8 }>());
        assert!(string_ptr.is::<{ tags::StringPtr }>());

        assert_eq!(format!("{:?}", string_ptr), "StringPtr(\"foo\")");

        assert_eq!(
            string_ptr.borrow_value::<StringPtr, String>(),
            &String::from("foo")
        );

        let clone = string_ptr.clone();
        assert_eq!(clone.unwrap::<StringPtr>().as_ref(), &String::from("foo"));
    }
}
