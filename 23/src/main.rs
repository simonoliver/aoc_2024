use std::collections::HashMap;
use std::fs;

fn main() {
    //solve("data/input");
    solve("data/test_input");
}

fn triangle_starts_with_char(triangle:&(&str,&str,&str),char_check:char) -> bool {
    triangle.0.chars().next().unwrap()==char_check || triangle.1.chars().next().unwrap()==char_check || triangle.2.chars().next().unwrap()==char_check
}
/*
fn check_or_insert_triangle_set(triangle_sets:&mut Vec<(&str,&str,&str)>,node_ids:(&str,&str,&str)) {
    let mut sorted_nodes:Vec<&str>=vec!{node_ids.0,node_ids.1,node_ids.2};
    sorted_nodes.sort();
    let triangle_tuple=(sorted_nodes[0],sorted_nodes[1],sorted_nodes[2]);
    if !triangle_sets.contains(&triangle_tuple)
    {
        triangle_sets.push(triangle_tuple);
    }
}
 */

fn solve(data_path:&str)
{
    let content = fs::read_to_string(data_path).expect("Expected to read the file");
    let lines=content.split("\n").filter(|line|!line.is_empty());
    let mut connection_pairs:HashMap<(&str,&str),bool>=HashMap::new(); // Whether connection exists
    let mut connection_sets:HashMap<&str,Vec<&str>> =HashMap::new(); // All connections for a given node
    for line in lines {
        let mut pair_split=line.split("-");
        let (node_id_0,node_id_1) = (pair_split.next().unwrap(),pair_split.next().unwrap());
        connection_pairs.entry((node_id_0,node_id_1)).or_insert(true); // Forward
        connection_pairs.entry((node_id_1,node_id_0)).or_insert(true); // Backward
        connection_sets.entry(node_id_0).and_modify(|value|value.push(node_id_1)).or_insert(vec!{node_id_1}); // Add entry
        connection_sets.entry(node_id_1).and_modify(|value|value.push(node_id_0)).or_insert(vec!{node_id_0}); // Add entry
    }
    // Check connections
    let mut triangle_sets:Vec<(&str,&str,&str)>=Vec::new(); // Vector of array of 3 items, for comparison]
    for node_connection_pair in &connection_sets {
        for &second_node_connection_id in node_connection_pair.1 { // Cycle through every node connected to "root" node
            let second_node_connection_set=connection_sets.get(&second_node_connection_id).unwrap();
            for third_node_connection_id in second_node_connection_set {
                if connection_pairs.contains_key(&(*third_node_connection_id,*node_connection_pair.0)) {
                    // Has a connection back. insert if not a duplicate
                    //check_or_insert_triangle_set(&mut triangle_sets,(*node_connection_pair.0,second_node_connection_id,third_node_connection_id))
                    let mut sorted_nodes:Vec<&str>=vec!{*node_connection_pair.0,second_node_connection_id,third_node_connection_id};
                    sorted_nodes.sort();
                    let triangle_tuple=(sorted_nodes[0],sorted_nodes[1],sorted_nodes[2]);
                    if !triangle_sets.contains(&triangle_tuple)
                    {
                        triangle_sets.push(triangle_tuple);
                    }
                }
            }
        }
    }
    println!("triangle sets found {}",triangle_sets.len());
    let filtered_sets:Vec<&(&str,&str,&str)>=triangle_sets.iter().filter(|triangle_set|triangle_starts_with_char(triangle_set,'t')).collect();
    println!("filtered_sets {}",filtered_sets.len());
}

