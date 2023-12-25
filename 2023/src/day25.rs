use std::cmp::min;
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxHashMap, FxHashSet};

#[aoc_generator(day25)]
fn parse(input: &str) -> (FxHashMap<String, Vec<String>>, FxHashSet<String>) {
    let mut result = FxHashMap::default();
    let mut vertexes = FxHashSet::default();

    for line in input.lines() {
        let (node, edges) = line.split_once(':').unwrap();
        let edges = edges.trim().split(' ');
        let mut vec = vec![];
        for edge in edges {
            vec.push(edge.to_string());
            vertexes.insert(edge.to_string());
        }
        vertexes.insert(node.to_string());
        result.insert(node.to_string(), vec);
    }
    (result, vertexes)
}

fn global_min_cut(edges: &mut [Vec<i32>]) -> (u32, Vec<u32>) {
    // https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
    let n = edges.len();
    let mut best = (u32::MAX, vec![]);

    let mut co: Vec<Vec<u32>> = vec![vec![]; n];
    co.iter_mut().enumerate().for_each(|(index, vec)| vec.push(index as u32));

    for ph in 1..n {
        let mut w = edges[0].clone();
        let mut s: usize = 0;
        let mut t: usize = 0;
        for _ in 0..n - ph {
            w[t] = i32::MIN;
            s = t;
            let max = *w.iter().max().unwrap();
            t = w.iter().position(|&v| v == max).unwrap();
            for (i, value) in w.iter_mut().enumerate() {
                *value += edges[t][i];
            }
        }
        best = min(best, ((w[t] - edges[t][t]) as u32, co[t].clone()));
        let temp = co[t].clone();
        co[s].extend(temp);

        for i in 0..n {
            edges[s][i] += edges[t][i];
        }
        for i in 0..n {
            edges[i][s] = edges[s][i];
        }
        edges[0][t] = i32::MIN;
    }
    best
}

#[aoc(day25, part1)]
fn part1(input: &(FxHashMap<String, Vec<String>>, FxHashSet<String>)) -> usize {
    let (graph, vertexes) = input;
    // Convert the graph to a adjacency matrix
    let n = vertexes.len();
    let mut edges = vec![vec![0; n]; n];
    for (k, v) in graph {
        let i1 = vertexes.iter().position(|key| key == k).unwrap();
        for edge in v {
            let i2 = vertexes.iter().position(|key| key == edge).unwrap();
            edges[i1][i2] = 1;
            edges[i2][i1] = 1;
        }
    }

    let (_, vec) = global_min_cut(&mut edges);
    vec.len() * (vertexes.len() - vec.len())
}

#[aoc(day25, part2)]
fn part2(_: &(FxHashMap<String, Vec<String>>, FxHashSet<String>)) -> String {
    "Woooow got the 50 stars".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 54);
    }
}