use std::mem::transmute;

#[macro_export]
macro_rules! make_thiscall_fn {
    ($address:expr, $returntype:ty) => {
        std::mem::transmute::<*const usize,extern "thiscall" fn() -> $returntype>($address as *const usize)
    };
    ($address:expr, $returntype:ty, $($argument:ty),*) => {
        std::mem::transmute::<*const usize,extern "thiscall" fn($($argument,)*) -> $returntype>($address as *const usize)
    }
}

pub(crate) use make_thiscall_fn;