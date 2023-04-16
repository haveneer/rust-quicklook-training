mod checkers {
    pub fn is_fn<A, R>(_x: fn(A) -> R) {}

    #[allow(non_snake_case)]
    pub fn is_Fn<A, R>(_x: &impl Fn(A) -> R) {}

    #[allow(non_snake_case)]
    pub fn is_FnMut<A, R>(_x: &impl FnMut(A) -> R) {}

    #[allow(non_snake_case)]
    pub fn is_FnOnce<A, R>(_x: &impl FnOnce(A) -> R) {}
}
use checkers::*;

#[test]
fn test_simple_fn() {
    fn simple_fn(i: i32) -> i32 {
        i
    }
    is_fn(simple_fn);
    is_Fn(&simple_fn);
    is_FnMut(&simple_fn);
    is_FnOnce(&simple_fn);
}

#[test]
fn test_no_context_fn() {
    let local_fn = |i: i32| i;
    is_fn(local_fn);
    is_Fn(&local_fn);
    is_FnMut(&local_fn);
    is_FnOnce(&local_fn);
}

struct Context;

impl Context {
    fn by_ref(&self, i: i32) -> i32 {
        i + 1
    }
    fn by_mut(&mut self, i: i32) -> i32 {
        i + 2
    }
    fn by_move(self, i: i32) -> i32 {
        i + 3
    }
}

#[test]
fn test_ref_context_fn() {
    let c = Context;
    let local_fn = |i: i32| c.by_ref(i);
    // is_fn(local_fn);
    is_Fn(&local_fn);
    is_FnMut(&local_fn);
    is_FnOnce(&local_fn);
}

#[test]
fn test_mut_context_fn() {
    let mut c = Context;
    let local_fn = |i: i32| c.by_mut(i);
    // is_fn(local_fn);
    // is_Fn(&local_fn);
    is_FnMut(&local_fn);
    is_FnOnce(&local_fn);
}

#[test]
fn test_context_fn() {
    let c = Context;
    let local_fn = |i: i32| c.by_move(i);
    // is_fn(local_fn);
    // is_Fn(&local_fn);
    // is_FnMut(&local_fn);
    is_FnOnce(&local_fn);
}
