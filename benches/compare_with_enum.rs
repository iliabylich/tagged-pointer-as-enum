use tagged_pointer_as_enum::{tagged_enum, TaggedPointerValue};

#[cfg(feature = "bench")]
extern crate criterion;

#[cfg(feature = "bench")]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct None;
impl TaggedPointerValue for None {}

enum Native {
    None(None),

    U8(Box<u8>),
    U16(Box<u16>),
    U32(Box<u32>),
    U64(Box<u64>),
    I8(Box<i8>),
    I16(Box<i16>),
    I32(Box<i32>),
}

impl Default for Native {
    fn default() -> Self {
        Native::None(None)
    }
}

tagged_enum! {
    enum Tagged {
        bits = 8;

        None(None),

        U8(Box<u8>),
        U16(Box<u16>),
        U32(Box<u32>),
        U64(Box<u64>),
        I8(Box<i8>),
        I16(Box<i16>),
        I32(Box<i32>),
    }
}

impl Default for Tagged {
    fn default() -> Self {
        Tagged::None(None)
    }
}

trait ForEachVariant {
    fn for_each_variant<F>(f: F)
    where
        F: FnMut(Self),
        Self: Sized;
}

impl ForEachVariant for Native {
    fn for_each_variant<F>(mut f: F)
    where
        F: FnMut(Self),
    {
        f(Native::U8(Box::new(10)));
        f(Native::U16(Box::new(1000)));
        f(Native::U32(Box::new(100_000)));
        f(Native::U64(Box::new(100_000)));
        f(Native::I8(Box::new(-10)));
        f(Native::I16(Box::new(-1000)));
        f(Native::I32(Box::new(-100_000)));
    }
}

impl ForEachVariant for Tagged {
    fn for_each_variant<F>(mut f: F)
    where
        F: FnMut(Self),
    {
        f(Tagged::U8(Box::new(10)));
        f(Tagged::U16(Box::new(1000)));
        f(Tagged::U32(Box::new(100_000)));
        f(Tagged::U64(Box::new(100_000)));
        f(Tagged::I8(Box::new(-10)));
        f(Tagged::I16(Box::new(-1000)));
        f(Tagged::I32(Box::new(-100_000)));
    }
}

fn work<T>()
where
    T: ForEachVariant + Default,
{
    let mut stack = Vec::with_capacity(7 * 50);

    for _ in 0..50 {
        T::for_each_variant(|v| {
            stack.push(v);
            let len = stack.len();
            std::mem::take(&mut stack[len - 1]);
        });
    }

    #[cfg(feature = "bench")]
    black_box(stack);
}

#[inline(never)]
#[cfg(feature = "bench")]
fn native_enum(c: &mut Criterion) {
    c.bench_function("native_enum", |b| b.iter(|| work::<Native>()));
}

#[inline(never)]
#[cfg(feature = "bench")]
fn tagged_enum(c: &mut Criterion) {
    c.bench_function("tagged_enum", |b| b.iter(|| work::<Tagged>()));
}

#[cfg(feature = "bench")]
criterion_group!(alloc, native_enum, tagged_enum);
#[cfg(feature = "bench")]
criterion_main!(alloc);

#[cfg(not(feature = "bench"))]
fn main() {}
