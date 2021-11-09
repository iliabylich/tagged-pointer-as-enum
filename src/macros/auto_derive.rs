#[macro_export]
#[doc(hidden)]
macro_rules! __derive_macros {
    ( $enum:ident ; $trait:ident, $($traits:ident,)* ; $($name:ident : $t:ty),* ) => {
        $crate::__derive_macro!($enum; $trait; $($name : $t),*);
        $crate::__derive_macros!($enum; $($traits,)* ; $($name: $t),*);
    };

    ( $enum:ident; ; $($name:ident : $t:ty),* ) => {
        // leaf, exit
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __derive_macro {
    ( $enum:ident; Debug; $($name:ident : $t:ty),* ) => {
        impl std::fmt::Debug for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let tag = self.tag();
                if tag == 0 {
                    panic!("Can't format empty TaggedPointer");
                }
                $(
                    if tag == tags::$name {
                        return self.pointer.format_as::<$t>(stringify!($name), f);
                    }
                )*
                panic!("Unknown tag {}", tag)
            }
        }
    };
    ( $enum:ident; Drop; $($name:ident : $t:ty),* ) => {
        impl Drop for $enum {
            fn drop(&mut self) {
                let tag = self.tag();
                if tag == 0 {
                    // empty TaggedPointer
                    return;
                }
                $(
                    if tag == tags::$name {
                        return self.pointer.drop_as::<$t>();
                    }
                )*
                panic!("Unknown tag {}", tag)
            }
        }
    };
    ( $enum:ident; Clone; $($name:ident : $t:ty),* ) => {
        impl Clone for $enum {
            fn clone(&self) -> Self {
                let tag = self.tag();
                if tag == 0 {
                    panic!("Can't clone empty TaggedPointer");
                }
                $(
                    if tag == tags::$name {
                        return Self { pointer: self.pointer.clone_as::<$t>() };
                    }
                )*
                panic!("Unknown tag {}", tag)
            }
        }
    };
    ( $enum:ident; PartialEq; $($name:ident : $t:ty),* ) => {
        impl PartialEq for $enum {
            fn eq(&self, other: &Self) -> bool {
                let l_tag = self.tag();
                let r_tag = other.tag();

                if l_tag == 0 {
                    panic!("Can't compare tagged pointers: lhs is empty");
                }
                if r_tag == 0 {
                    panic!("Can't compare tagged pointers: rhs is empty");
                }

                $(
                    if l_tag == tags::$name && r_tag == tags::$name {
                        return self.pointer.compare_as::<$t>(&other.pointer);
                    }
                )*

                false
            }
        }
    };
    ( $enum:ident; Eq; $($name:ident : $t:ty),* ) => {
        impl Eq for $enum {}
    };
    ( $enum:ident; $d:ident; $($name:ident : $t:ty),* ) => {
        compile_error!(stringify!($d));
        compile_error!("Auto-deriving unsupported trait (only `Debug`, `Clone`, `PartialEq` and `Eq` are supported");
    };
}
