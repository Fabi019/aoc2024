use std::collections::{HashMap, HashSet};

aoc2024::main!("../../assets/day23.txt");

fn part1(input: &str) -> u32 {
    let mut network: HashMap<_, HashSet<_>> = HashMap::new();

    for con in input.lines() {
        let (a, b) = con.split_once("-").unwrap();
        network.entry(a).or_default().insert(b);
        network.entry(b).or_default().insert(a);
    }

    let nodes = network.keys();
    let mut groups = HashSet::new();

    for node in nodes {
        let other = &network[node];

        for a in other {
            let this = &network[a];

            for b in other {
                if a != b && this.contains(b) {
                    let mut group = vec![node, a, b];
                    group.sort_unstable();
                    groups.insert(group);
                }
            }
        }
    }

    let mut count = 0;

    for group in groups {
        if group.iter().any(|n| n.starts_with("t")) {
            count += 1;
        }
    }

    count
}

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    graph: &'a HashMap<&str, HashSet<&str>>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    for v in p.clone() {
        let mut r_new = r.clone();
        r_new.insert(v);

        let mut p_new = p.intersection(&graph[&v]).copied().collect();
        let mut x_new = x.intersection(&graph[&v]).copied().collect();
        bron_kerbosch(r_new, &mut p_new, &mut x_new, graph, cliques);

        p.remove(v);
        x.insert(v);
    }
}

fn part2(input: &str) -> String {
    let mut network: HashMap<_, HashSet<_>> = HashMap::new();

    for con in input.lines() {
        let (a, b) = con.split_once("-").unwrap();
        network.entry(a).or_default().insert(b);
        network.entry(b).or_default().insert(a);
    }

    let mut cliques = Vec::new();

    bron_kerbosch(
        HashSet::new(),
        &mut network.keys().copied().collect(),
        &mut HashSet::new(),
        &network,
        &mut cliques,
    );

    let largest = cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap_or_default();

    let mut largest = Vec::from_iter(largest);
    largest.sort_unstable();

    largest.join(",")
}

aoc2024::test!(
    "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
",
    7,
    "co,de,ka,ta"
);
