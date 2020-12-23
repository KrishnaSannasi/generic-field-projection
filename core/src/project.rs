use typsy::cmp::Any;

use super::*;

#[cfg(feature = "alloc")]
pub mod from_arc;
#[cfg(feature = "alloc")]
pub mod from_box;
pub mod from_mut;
pub mod from_pin;
#[cfg(feature = "alloc")]
pub mod from_rc;
pub mod from_ref;

use core::{marker::PhantomData, ops::Deref, pin::Pin};

use crate::pin::*;

pub struct PtrToRef<'a>(PhantomData<&'a ()>);

typsy::call! {
    fn['a, T: 'a + Sized](&mut self: PtrToRef<'a>, ptr: *const T) -> &'a T {
        unsafe { &*ptr }
    }
}

pub struct FindOverlap<S> {
    counter: u64,
    set:     S,
}

impl<S> FindOverlap<S> {
    fn new(set: S) -> Self {
        FindOverlap {
            set,
            counter: 0,
        }
    }
}

pub struct FindOverlapInner<I> {
    id:      u64,
    counter: u64,
    field:   I,
}

typsy::call! {
    fn[S, F](&mut self: FindOverlap<S>, field: F) -> bool
    where(
        S: Copy + for<'b> Any<'b, FindOverlapInner<F>>,
        F: Field,
    ){
        self.counter += 1;

        self.set.any(FindOverlapInner {
            id: self.counter,
            counter: 0,
            field
        })
    }

    fn[I: Field, J: Field](&mut self: FindOverlapInner<I>, input: J) -> bool
    where(
        I::Parent: Sized,
        J::Parent: Sized,
        I::Type: Sized,
        J::Type: Sized,
    ){
        self.counter += 1;

        if self.id <= self.counter {
            return false
        }

        let field = self.field.range();
        let input = input.range();

        is_overlapping(field, input)
    }
}

fn is_overlapping(a: Range<usize>, b: Range<usize>) -> bool {
    a.contains(&b.start) || a.contains(&b.end)
}
