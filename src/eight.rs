use std::fs;

#[derive(Debug)]
struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>,
}

fn parse_node(license_data_buffer: &mut Vec<u32>) -> Node {
    // Specifically, a node consists of:
    // * A header, which is always exactly two numbers:
    //     * The quantity of child nodes.
    //     * The quantity of metadata entries.
    let children_count = license_data_buffer.pop().unwrap();
    let metadata_count = license_data_buffer.pop().unwrap();

    let mut node = Node {
        metadata: vec![],
        children: vec![],
    };

    // * Zero or more child nodes (as specified in the header).
    for _ in 0..children_count {
        node.children.push(parse_node(license_data_buffer));
    }

    // * One or more metadata entries (as specified in the header).
    for _ in 0..metadata_count {
        node.metadata.push(license_data_buffer.pop().unwrap());
    }

    node
}

fn metadata_values(node: Node) -> Vec<u32> {
    let mut ret = node.metadata.clone();

    for child in node.children {
        ret.extend(metadata_values(child));
    }

    ret
}

/// The first check done on the license file is to simply add up all of the metadata entries.
pub fn eight_a() -> u32 {
    let contents = fs::read_to_string("src/inputs/8.txt").unwrap();
    let mut license_data_buffer: Vec<u32> = contents
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    license_data_buffer.reverse();

    let node = parse_node(&mut license_data_buffer);

    metadata_values(node).iter().sum()
}

/// The second check is slightly more complicated: you need to find the value of the root node.
/// The value of a node depends on whether it has child nodes.
/// If a node has no child nodes, its value is the sum of its metadata entries.
/// However, if a node does have child nodes, the metadata entries become indexes which refer
/// to those child nodes. The value of this node is the sum of the values of the child nodes
/// referenced by the metadata entries. If a referenced child node does not exist, that reference
/// is skipped. A child node can be referenced multiple time and counts each time it is referenced.
/// A metadata entry of 0 does not refer to any child node.
fn node_value(node: &Node) -> u32 {
    if node.children.is_empty() {
        node.metadata.iter().sum()
    } else {
        let mut sum = 0;

        for &index in &node.metadata {
            // A metadata entry of 1 refers to the first child node, 2 to the second,
            // 3 to the third, and so on.
            let index: usize = index as usize - 1;

            if index < node.children.len() {
                sum += node_value(&node.children[index]);
            }
        }

        sum
    }
}

/// What is the value of the root node?
pub fn eight_b() -> u32 {
    let contents = fs::read_to_string("src/inputs/8.txt").unwrap();
    let mut license_data_buffer: Vec<u32> = contents
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    license_data_buffer.reverse();

    let node = parse_node(&mut license_data_buffer);

    node_value(&node)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(eight_a(), 40309);
        assert_eq!(eight_b(), 28779);
    }
}
