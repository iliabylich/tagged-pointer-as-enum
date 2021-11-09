#[macro_export]
#[doc(hidden)]
macro_rules! __declare_tags {
    ( start = $n:expr; $vis:vis ) => {};
    ( start = $n:expr; $vis:vis $x:ident $(, $name:ident)* ) => {
        pub(crate) const $x: usize = $n;
        $crate::__declare_tags!(start = $n + 1; $vis $($name),* );
    };
}

#[test]
fn test_declare_tags() {
    __declare_tags!(start = 42; pub A, B, C);
    assert_eq!(A, 42);
    assert_eq!(B, 43);
    assert_eq!(C, 44);
}
