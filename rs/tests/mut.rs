#[cfg(test)]
mod tests {
    fn f1(v : Vec<i32>) -> Vec<i32> {
        v
    }
    
    #[test]
    fn test_moved_no_change() {
        let v1 = vec!{1,2,3};
        let v2 = f1(v1); // v1 moved into f1
        // println!("v1 has length {} content `{:?}`", v1.len(), v1); // v1 no more available
        println!("v2 has length {} content `{:?}`", v2.len(), v2);
    }

    #[test]
    fn test_moved() {
        let v1 = vec!{1,2,3};
        let mut v2 = f1(v1.clone());
        println!("v1 has length {} content `{:?}`", v1.len(), v1);
        v2.push(5);
        println!("v2 has length {} content `{:?}`", v2.len(), v2);
    }

    fn f2(mut v : Vec<i32>) -> Vec<i32> { // moved as mutable
        v.push(4);
        v
    }

    #[test]
    fn test_moved_as_mut() {
        let v1 = vec!{1,2,3};
        let mut v2 = f2(v1);
        // println!("v1 has length {} content `{:?}`", v1.len(), v1); // v1 no more available
        v2.push(5);
        println!("v2 has length {} content `{:?}`", v2.len(), v2);
    }

    fn f3(v : &Vec<i32>) -> Vec<i32> { // borrowed
        let mut v = v.clone(); // required to do mutate content
        v.push(4);
        v
    }

    #[test]
    fn test_borrowed() {
        let v1 = vec!{1,2,3};
        let mut v2 = f3(&v1); // v1 just borrowed, not moved
        println!("v1 has length {} content `{:?}`", v1.len(), v1);
        v2.push(5);
        println!("v2 has length {} content `{:?}`", v2.len(), v2);
    }

    fn f4(v : &Vec<i32>) -> Vec<i32> { // borrowed
        let mut v = v.clone(); // required to do mutate content
        v.push(4);
        v
    }

    #[test]
    fn test_mut_in_place() {
        let mut v1 = vec!{1,2,3};
        f4(&mut v1); // borrowed as mutable
        v1.push(5);
        println!("v1 has length {} content `{:?}`", v1.len(), v1);
    }
}