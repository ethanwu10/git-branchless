use criterion::{black_box, criterion_group, criterion_main, Criterion};

use scm_bisect::testing::UsizeGraph;
use scm_bisect::{basic_search, search};

fn bench_search(
    graph: UsizeGraph,
    strategy: impl search::Strategy<UsizeGraph>,
    speculation_size: usize,
) -> search::Bounds<usize> {
    let is_problem = |node: usize| node <= 30;

    let mut search = search::Search::new(graph);
    let bounds = loop {
        let search_nodes = {
            let search::LazySolution {
                bounds,
                next_to_search,
            } = search.search(&strategy).unwrap();
            let search_nodes = next_to_search
                .take(speculation_size)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            if search_nodes.is_empty() {
                break bounds.clone();
            } else {
                search_nodes
            }
        };
        for search_node in search_nodes {
            let status = if is_problem(search_node) {
                search::Status::Failure
            } else {
                search::Status::Success
            };
            search.notify(search_node, status).unwrap();
        }
    };
    bounds
}

fn bench_search_per_graph_size(c: &mut Criterion) {
    // @nocommit increase graph size significantly
    for graph_size in [4, 8, 32, 128, 1024, 8192, 65536, 1_000_000] {
        let graph = UsizeGraph { max: graph_size };
        let strategy = basic_search::BasicStrategy::new(basic_search::BasicStrategyKind::Binary);
        c.bench_function(&format!("bench_search_graph_size_{graph_size}"), |b| {
            b.iter(|| black_box(bench_search(graph.clone(), strategy.clone(), 1)))
        });
    }
}

// @nocommit: fix slow speculation
fn bench_search_per_speculation_size(c: &mut Criterion) {
    for speculation_size in [1, 4, 8, 32, 128] {
        let graph = UsizeGraph { max: 32 };
        let strategy = basic_search::BasicStrategy::new(basic_search::BasicStrategyKind::Binary);
        c.bench_function(
            &format!("bench_search_speculation_size_{speculation_size}"),
            |b| {
                b.iter(|| {
                    black_box(bench_search(
                        graph.clone(),
                        strategy.clone(),
                        speculation_size,
                    ))
                })
            },
        );
    }
}

criterion_group!(
    benches,
    bench_search_per_graph_size,
    bench_search_per_speculation_size
);
criterion_main!(benches);
