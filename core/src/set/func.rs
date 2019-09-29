use crate::set::tuple::TypeFunction;
use core::marker::PhantomData;

pub struct PtrToRef<'a>(PhantomData<&'a ()>);

impl<'a> PtrToRef<'a> {
    pub(crate) unsafe fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T: ?Sized + 'a> TypeFunction<*const T> for PtrToRef<'a> {
    type Output = &'a T;

    fn call(&mut self, input: *const T) -> Self::Output {
        unsafe { &*input }
    }
}

pub struct PtrToRefMut<'a>(PhantomData<&'a ()>);

impl<'a> PtrToRefMut<'a> {
    pub(crate) unsafe fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T: ?Sized + 'a> TypeFunction<*mut T> for PtrToRefMut<'a> {
    type Output = &'a mut T;

    fn call(&mut self, input: *mut T) -> Self::Output {
        unsafe { &mut *input }
    }
}
