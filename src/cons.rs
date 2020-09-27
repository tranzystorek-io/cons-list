use crate::list::List;

macro_rules! head_method_body {
    ($myself:ident) => {
        match $myself {
            Cons::Cons(head, _) => Some(head),
            _ => None,
        }
    };
}

macro_rules! tail_method_body {
    ($myself:ident) => {
        match $myself {
            Cons::Cons(_, tail) if !tail.is_empty() => Some(tail),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! head_matches {
    ($cons:expr, $($head:pat)|+ $( if $guard:expr )?) => {
        match $cons {
            $( $crate::Cons::Cons($head, _) )|+ $( if $guard )? => true,
            _ => false
        }
    };
}

pub type LCons<T> = Cons<T, List<T>>;

pub enum Cons<T, L> {
    Cons(T, L),
    Nil,
}

impl<T> Cons<T, List<T>> {
    pub fn head(self) -> Option<T> {
        head_method_body!(self)
    }

    pub fn as_head(&self) -> Option<&T> {
        head_method_body!(self)
    }

    pub fn as_mut_head(&mut self) -> Option<&mut T> {
        head_method_body!(self)
    }

    pub fn tail(self) -> Option<List<T>> {
        tail_method_body!(self)
    }

    pub fn as_tail(&self) -> Option<&List<T>> {
        tail_method_body!(self)
    }

    pub fn as_mut_tail(&mut self) -> Option<&mut List<T>> {
        tail_method_body!(self)
    }

    pub fn as_ref(&self) -> Cons<&T, &List<T>> {
        match *self {
            Cons::Cons(ref head, ref tail) => Cons::Cons(head, tail),
            _ => Cons::Nil,
        }
    }
}

impl<T, L> Cons<T, L> {
    pub fn is_cons(&self) -> bool {
        matches!(self, Cons::Cons(_, _))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Cons::Nil)
    }
}
