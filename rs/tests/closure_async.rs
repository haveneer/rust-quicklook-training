use futures::FutureExt;
use std::future::ready;
use tokio::fs;

pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

async fn process<Reader, Writer>(files: &[&str], reader: Reader, mut writer: Writer)
where
    // F : async FnOnce() -> String // syntax not yey stabilized
    Reader: AsyncFn(&str) -> String,
    Writer: AsyncFnMut(String),
{
    for f in files {
        let s = reader(f).await; // TODO let's do it with `then` & cie
        println!("s is {}", s.type_name());
        writer(s).await;
    }
}

#[tokio::test]
async fn main() {
    let reader = async |path: &str| {
        let r = fs::read_to_string(&path).await.unwrap();
        r
    };

    let x = fs::read_to_string(&file!());

    let mut vec: Vec<String> = vec![];
    let writer = async |s: String| {
        vec.push(ready(s).await);
    };

    let files = [file!(), file!(), file!()];
    process(&files, reader, writer).await;
    println!("{:?}", vec.iter().map(|s| s.len()).collect::<Vec<_>>());
}
