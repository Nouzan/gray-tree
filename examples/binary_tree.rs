use gray_tree::{binary_tree::Node, Result};

fn main() -> Result<()> {
    //        1
    //       / \
    //      /   \
    //     /     \
    //    /       \
    //    2       3
    //   / \     / \
    //  /   \   /   \
    //  4   5   6   7
    //     /     \
    //     8     9

    let left = {
        let left = Node::builder().data(4).build()?;
        let right = {
            let left = Node::new(8);
            Node::builder().data(5).left(left).build()?
        };
        Node::builder().data(2).left(left).right(right).build()?
    };
    let right = {
        let left = {
            let right = Node::new(9);
            Node::builder().data(6).right(right).build()?
        };
        let right = Node::new(7);
        Node::builder().data(3).left(left).right(right).build()?
    };
    let root = Node::builder().data(1).left(left).right(right).build()?;

    let root = root.post_order_map(|node| {
        println!("{}", node);
        node
    });

    let mut iter = root.level_order_iter();
    let mut cached = 0;
    while let Some((level, data)) = iter.next() {
        if level > cached {
            println!();
            cached = level;
        }
        print!("{} ", data);
    }
    println!();

    println!("{}", root);

    Ok(())
}
