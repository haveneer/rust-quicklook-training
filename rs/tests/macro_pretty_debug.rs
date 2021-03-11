// Declarative matching macros
macro_rules! debug {
    (msg:$msg:literal $($x:expr);+) => {{ // the order matters !
        let (name_width, value_width, type_width, size_width) = max_name_size!($(&$x);+);
        let width = std::cmp::max(30, name_width+value_width+type_width+size_width+9);
        println!("{:=^width$}", [" ",$msg," "].concat(), width=width);
        $(
            println!("{:width1$} = {:width2$} : {:width3$} ({})",
                     std::stringify!($x),$x, get_type_string(&$x), get_type_size(&$x),
                     width1=name_width, width2=value_width, width3=type_width);
        )*
    }};
    ($($x:expr);+) => {{ // use ; separator (not ,)
        debug!(msg:"Debug Info" $($x);+);
    }};
}

macro_rules! max_name_size { // Loop recursively
    () => { 0 };
    ($head:expr) => {
        (std::stringify!($head).len()-1, ($head).to_string().len(), get_type_string($head).len(), get_type_size($head).to_string().len())
    };
    ($head:expr; $($tail:expr);*) => {
        std::cmp::max(max_name_size!($head), max_name_size!($($tail);*))
    };
}

fn get_type_string<T>(_t: &T) -> &'static str {
    std::any::type_name::<T>()
}
fn get_type_size<T>(_t: &T) -> usize {
    std::mem::size_of::<T>()
}

#[test]
fn test_debug() {
    let var = 1;
    let something_else = "string";
    debug!(var);
    debug!(var; something_else);
    debug!(1+2; something_else);

    let str = Struct { field: 42 };
    debug!(msg:"Hello" var; str );

    println!("\nOk not moved : {}", str);
}

// Not copiable structure
#[derive(Debug)]
struct Struct {
    field: i64,
}

impl std::fmt::Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
