enum Tree {
    Leaf(i32),
    Node {
        left: Box<Tree>,
        payload: i32,
        right: Box<Tree>,
    },
}

fn sample_tree() -> Tree {
    let l1 = Box::new(Tree::Leaf(1));
    let l3 = Box::new(Tree::Leaf(3));
    let n2 = Box::new(Tree::Node {
        left: l1,
        payload: 2,
        right: l3,
    });
    let l5 = Box::new(Tree::Leaf(5));

    // Returns tree that Looks like:
    //
    //      +----(4)---+
    //      |          |
    //   +-(2)-+      [5]
    //   |     |
    //  [1]   [3]
    //
    Tree::Node {
        left: n2,
        payload: 4,
        right: l5,
    }
}

fn tree_weight(t: &Tree) -> i32 {
    match *t {
        Tree::Leaf(payload) => payload,
        Tree::Node {
            ref left,
            payload,
            ref right,
        } => {
            tree_weight(left) + payload + tree_weight(right)
            // or without 'ref' on declaration and no *t dereference and *payload
        }
    }
}

fn main() {
    let tree = sample_tree();
    assert_eq!(tree_weight(&tree), (1 + 2 + 3) + 4 + 5);
    println!("weight = {}", tree_weight(&tree));
}

#[test]
fn test() {
    main()
}
