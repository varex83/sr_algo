use rand::random;
use sr_algo::bst::Node;

fn main() {
    let mut root = Node::new(50);
    for _ in 0..100 {
        root.insert(random::<u32>() % 101);
    }

    println!("Preorder traversal:");
    root.preorder();
    println!();

    println!("Inorder traversal:");
    root.inorder();
    println!();

    println!("Postorder traversal:");
    root.postorder();
    println!();
}
