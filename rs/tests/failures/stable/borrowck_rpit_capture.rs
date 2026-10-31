fn indices(users: &[String]) -> impl Iterator<Item = usize> {
    0..users.len()
}

fn main() {
    let mut users = vec![String::from("l.roux")];
    let range = indices(&users);
    users.push(String::from("a.martin"));
    println!("{}", range.count());
}
