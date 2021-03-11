#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let i = 2;
        let j = i; // HINT copyable data
        println!("i = {}", i);
        println!("j = {}", j);
    }

    #[test]
    fn test2() {
        let i = "Hello";
        let j = i; //
        println!("i = {}", i);
        println!("j = {}", j);
    }

    #[test]
    fn test3() {
        let i = String::from("Hello");
        let j = i.clone(); // TODO why clone() ??
        println!("i = {}", i);
        println!("j = {}", j);
    }

    #[test]
    fn test4() {
        #[derive(Debug)]
        struct Command {
            name: String,
            run_async: bool,
        }

        let c1 = Command {
            name: "curl".to_string(),
            run_async: false,
        };
        let c2 = Command {
            name: c1.name.clone(),   // TODO what happens without clone() ?
            run_async: c1.run_async, // TODO why no clone() needed here ?
        };

        println!("c1.run_async = {:?}", c1.run_async);
        println!("c2.run_async = {:?}", c2.run_async);
        println!("c1 = {:?}", c1); // TODO without clone and without these two prints
        println!("c2 = {:?}", c2); //
    }
}
