use std::collections::{BTreeSet, HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

const INPUT: &str = include_str!(concat!("../../inputs/", module_path!(), ".txt"));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node<'a> {
    name: &'a str,
    flow: i64,
    adj: Vec<&'a str>,
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.flow.partial_cmp(&other.flow)
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.flow.cmp(&other.flow)
    }
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Node<'_>>> {
    let parse_line = map(
        tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            complete::i64,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnels lead to valve "),
                tag("; tunnel leads to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list0(tag(", "), alpha1),
        )),
        |(_, name, _, flow, _, adj)| Node { name, flow, adj },
    );
    separated_list0(tag("\n"), parse_line)(input)
        .map(|(i, nodes)| (i, nodes.into_iter().map(|node| (node.name, node)).collect()))
}

fn dfs<'a>(
    nodes: &HashMap<&'a str, Node<'a>>,
    open_valves: &BTreeSet<&'a str>,
    cache: &mut HashMap<String, i64>,
    time_left: i64,
    cur_node: &str,
    elephant: bool,
) -> i64 {
    let cur_node = nodes.get(cur_node).unwrap();
    let cache_key = format!(
        "{}:{}:{}:{}",
        open_valves.iter().fold(String::new(), |o, i| o + i),
        cur_node.name,
        time_left,
        elephant
    );

    if let Some(v) = cache.get(&cache_key) {
        return *v;
    }

    if time_left == 0 {
        let flow = if elephant {
            dfs(nodes, open_valves, cache, 26, "AA", false)
        } else {
            0
        };
        cache.insert(cache_key, flow);
        return flow;
    }

    let mut flow = 0i64;
    if !open_valves.contains(cur_node.name) && cur_node.flow > 0 {
        // Open a valve and spend 1 minute if it is worth it to do so.
        let mut open_valves = open_valves.clone();
        open_valves.insert(cur_node.name);
        let sub_flow = dfs(
            nodes,
            &open_valves,
            cache,
            time_left - 1,
            cur_node.name,
            elephant,
        );
        flow = i64::max(cur_node.flow * (time_left - 1) + sub_flow, flow);
    }

    // Move to another node.
    let flow = cur_node.adj.iter().fold(flow, |o, edge| {
        let sub_flow = dfs(nodes, open_valves, cache, time_left - 1, edge, elephant);
        i64::max(sub_flow, o)
    });

    cache.insert(cache_key, flow);

    flow
}

fn run1(input: &'static str) -> usize {
    let (_, nodes) = parse(input.trim()).unwrap();

    dfs(
        &nodes,
        &Default::default(),
        &mut Default::default(),
        30,
        "AA",
        false,
    ) as _
}

fn run2(input: &'static str) -> usize {
    let (_, nodes) = parse(input.trim()).unwrap();

    dfs(
        &nodes,
        &Default::default(),
        &mut Default::default(),
        26,
        "AA",
        true,
    ) as _
}

fn main() {
    dbg!(run1(INPUT.trim()));
    dbg!(run2(INPUT.trim()));
}

#[cfg(test)]
const SAMPLE01: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run1(SAMPLE01), 1651);
    }

    #[test]
    fn test2() {
        assert_eq!(run2(SAMPLE01), 1707);
    }
}
