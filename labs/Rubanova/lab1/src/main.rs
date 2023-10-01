use std::fs::File;
use std::io::{self, Read};
use std::collections::VecDeque;

// binary tree structure
#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn build_tree(values: &mut Vec<i32>) -> Option<Box<TreeNode>> {
    if values.is_empty() {
        return None;
    }

    // Pop the first value and create a TreeNode
    let value = values.remove(0);
    let left = build_tree(values);
    let right = build_tree(values);

    Some(Box::new(TreeNode { value, left, right }))
}

// bfs
fn bfs_traversal(root: &Option<Box<TreeNode>>) {
    if let Some(root) = root {
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                print!("{} ", node.value);

                if let Some(left) = &node.left {
                    queue.push_back(left);
                }

                if let Some(right) = &node.right {
                    queue.push_back(right);
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("tree.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut values: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let root = build_tree(&mut values);

    println!("BFS:");
    bfs_traversal(&root);
    println!();

    Ok(())
}