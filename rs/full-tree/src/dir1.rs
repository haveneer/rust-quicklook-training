// this file is related to the directory with the same name

pub mod f2; // pub to be transitive to which load this dir1 mod

pub fn run() {
    f2::f2_helper(); // ok: dir1 is the parent of f2
}
