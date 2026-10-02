fn total(v: Vec<f64>) -> f64 {
    v.iter().sum()
}

fn main() {
    let data = vec![1.0, 2.0, 3.0];
    let t = total(data);
    println!("{t} over {} values", data.len());
}
