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

// The first check done on the license file is to simply add up all of the metadata entries.
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
