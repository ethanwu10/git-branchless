use std::collections::HashSet;

use scm_bisect::minimize::{Minimize, Strategy, Subset};
use scm_bisect::search::{Bounds, LazySolution, Status};
use scm_bisect::testing::UsizeGraph;

fn example_minimize(max: usize) -> Bounds<Subset<usize>> {
    let graph = UsizeGraph { max };
    let nodes = 0..graph.max;
    let is_problem = |set: &HashSet<usize>| -> bool { set.contains(&2) && set.contains(&4) };
    // @nocommit: remove explicit type annotation once graph and strategy are merged
    let mut minimize = Minimize::<UsizeGraph>::new_with_nodes(nodes);

    let bounds = loop {
        let (search_node, status) = {
            let LazySolution {
                bounds,
                mut next_to_search,
            } = minimize.search(&Strategy::Add).unwrap();
            let search_node = match next_to_search.next() {
                Some(search_node) => search_node.unwrap(),
                None => break bounds.clone(),
            };
            let status = if is_problem(&search_node.iter().copied().collect()) {
                Status::Failure
            } else {
                Status::Success
            };
            (search_node, status)
        };
        minimize.notify(search_node, status).unwrap();
    };
    bounds
}

fn main() {
    let bounds = example_minimize(4);
    println!("bounds are {bounds:?}");
}
