mod dir1;
mod dir2;
mod f1; // declare an independent module (see f1.rs) // declare an independent module (see dir1.rs)

use mylib1; // declared in this project Cargo.tom

fn main() {
    {
        // direct usage
        f1::f1();
    }
    {
        // using a 'use' loader
        use f1::f1;
        f1();
    }

    {
        // direct usage
        crate::dir1::f2::f2(); // crate:: is optional
    }

    {
        // using a 'use' loader : path from use + path on usage should be the complete path
        use crate::dir1::f2;
        f2::f2();
    }

    {
        // use f7 as it is directly in dir2 (was forwarded in the module)
        dir2::f7();
    }

    {
        // direct usage (mylib1 crate is defined as dependency of current project)
        mylib1::f3::f3();
    }

    {
        // direct usage
        // mylib1::f4::f4(); // error: f4 is a private module
        mylib1::f4();
        mylib1::f5(); // an alias defined in mylib1
    }

    {
        // direct usage
        // mylib1::f6x::f6a(); // error: f6x is a private module
        mylib1::f6a();
        // mylib1::f6b(); // was restricted to mylib1 with pub(crate)
    }
}
