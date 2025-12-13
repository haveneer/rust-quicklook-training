use futures::future::try_join_all;
use futures::FutureExt;
use std::hash::{DefaultHasher, Hash, Hasher};

async fn process<Reader>(files: &[&str], reader: Reader) -> Result<String, std::io::Error>
where
    // F : async FnOnce() -> String // syntax not yet stabilized
    Reader: AsyncFn(&str) -> Result<String, std::io::Error>,
{
    // Lancer toutes les lectures en parallèle puis agréger les résultats
    let futures_iter = files
        .iter()
        .copied()
        .enumerate()
        .map(async |(_idx, filename)| reader(filename).await);
    let results: Vec<_> = try_join_all(futures_iter).await?;
    Ok(results.concat())
}

#[tokio::test]
async fn main() {
    let reader = async |path: &str| tokio::fs::read_to_string(&path).await;

    let async_hasher = async |s: String| -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    };

    let files = [file!(), file!(), file!()];
    let r = process(&files, reader)
        .then(async |x| x.unwrap())
        .then(async_hasher)
        .await; // Chaining
    println!("Hash: {r:08X}");
}
