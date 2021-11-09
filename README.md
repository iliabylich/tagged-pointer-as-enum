### tagged-pointer-as-enum

A set of structs, traits and macros to implement tagged pointers.

### Basic usage

```rust
// import macro
use tagged_pointer_as_enum::tagged_enum;

// declare it like a normal enum but inside tagged_enum! macro
tagged_enum! {
    enum E {
        // specify how many bits you want to be used by tag.
        // should be at least log2(count(variants) + 1), but you can set more
        // if you need to keep ABI stable after adding new variants
        bits = 2;

        A(u8),
        B(Box<String>),
        C(i16),
    }
}

// macro generates constructors:
let a = E::A(42_u8);
let b = E::B(Box::new(String::from("foo")));
let c = E::C(300_i16);

// and a helper module with tags
assert_eq!(tags::A, 1);
assert_eq!(tags::B, 2);
assert_eq!(tags::C, 3);

// these tags can be used to check variant of enum
assert_eq!(a.tag(), tags::A);
assert_eq!(b.tag(), tags::B);
assert_eq!(c.tag(), tags::C);

// only variants that behave like containers can be borrowed
assert_eq!(b.borrow_value::<Box<String>, String>(), &String::from("foo"));
// borrowing values variants is impossible
// because there's no memory location containing value WITHOUT tag

// of course, you can get values back
assert_eq!(a.unwrap::<u8>(), 42);
assert_eq!(b.unwrap::<Box<String>>(), Box::new(String::from("foo")));
assert_eq!(c.unwrap::<u16>(), 300);
```

### Custom variant types

By default the following types can be used as variants:

+ `u8`
+ `u16`
+ `u32`
+ `i8`
+ `i16`
+ `i32`
+ `Box<T>`
+ `Option<Box<T>>`

It is possible to use other types by implementing `TaggedPointerValue` for them:

```rust
use tagged_pointer_as_enum::{tagged_enum, TaggedPointerValue};

struct Custom {
    low: u8,
    high: u8
}

// even if it looks like a marker trait in fact it's not
impl TaggedPointerValue for Custom {}

tagged_enum! {
    enum E {
        bits = 1;

        Custom(Custom),
    }
}

let custom = E::Custom(Custom { low: 1, high: 2 });
let unwrapped = custom.unwrap::<Custom>();
assert_eq!(unwrapped.low, 1);
assert_eq!(unwrapped.high, 2);
```

### Implementing default traits

Out of the box `tagged_enum!` macro generates a struct that doesn't implement any builtin traits.

It is possible to attach `#[derive(...)]` metadata similar to how it's usually attached to native Rust enums, however all variant types must also implement these traits.

**Only the following traits are supported**

+ `Debug`
+ `Clone`
+ `PartialEq`
+ `Eq`

Also, `Drop` is auto-implemented for all tagged enums.

```rust
use tagged_pointer_as_enum::{tagged_enum, TaggedPointerValue};

#[derive(Debug)]
struct NumAbsCompare {
    n: i32
}
impl TaggedPointerValue for NumAbsCompare {}

// implement comparison by absolute value
impl PartialEq for NumAbsCompare {
    fn eq(&self, other: &Self) -> bool {
        self.n.abs() == other.n.abs()
    }
}

tagged_enum! {
    #[derive(Debug, PartialEq)]
    enum E {
        bits = 2;

        I32(i32),
        NumAbsCompare(NumAbsCompare),
    }
}

// variants ARE equal if they have the same tag and value
assert_eq!(
    E::NumAbsCompare(NumAbsCompare { n:  100 }),
    E::NumAbsCompare(NumAbsCompare { n: -100 }),
);
// variants ARE NOT equal if tags are different
assert_ne!(
    E::NumAbsCompare(NumAbsCompare { n: 100 }),
    E::I32(100),
);
// variants ARE NOT equal if values are different
assert_ne!(
    E::NumAbsCompare(NumAbsCompare { n: 100 }),
    E::NumAbsCompare(NumAbsCompare { n: 101 }),
);
```
