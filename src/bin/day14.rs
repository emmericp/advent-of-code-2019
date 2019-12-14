use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./input/day14.txt").unwrap();
    let reactions = parse_reactions(&input);
    dbg!(build_product("FUEL", 1, &reactions, &mut HashMap::new()));
    let mut spare_resources = HashMap::new();
    let mut ore_counter: i64 = 0;
    let mut fuel_counter: i64 = 0;
    // brute force, lol (~30 minutes). not using real value here to avoid CI slowness ;)
    // can't do anything more clever (binary search etc.) without optimizing build_product below,
    // the problem is that it doesn't batch reactions, i.e., asking for 1000 fuel adds 1000 entries to the internal list ;)
    let ore_target = 10_000_000; //1_000_000_000_000;
    while ore_counter < ore_target {
        ore_counter += build_product("FUEL", 1, &reactions, &mut spare_resources);
        fuel_counter += 1;
        if fuel_counter % 1_000 == 0 {
            println!(
                "{}% done, {} ore for {} fuel, estimate: {} fuel total",
                ore_counter * 100 / ore_target,
                ore_counter,
                fuel_counter,
                fuel_counter as f64 / ore_counter as f64 * ore_target as f64
            );
        }
    }
    dbg!(fuel_counter - 1);
}

fn build_product<'a>(
    product: &'a str,
    count: i32,
    reactions: &HashMap<&str, Reaction<'a>>,
    spare_resources: &mut HashMap<&'a str, i32>
) -> i64 {
    let mut ore_count = 0;
    let mut todo = vec![];
    todo.push((&product, count));
    while !todo.is_empty() {
        let (product, count) = todo.pop().unwrap();
        let spare = spare_resources.entry(*product).or_insert(0);
        if *spare >= count {
            *spare -= count
        } else {
            let reaction = reactions.get(product).unwrap();
            for input in reaction.inputs.iter() {
                if input.product == "ORE" {
                    ore_count += input.count;
                } else {
                    let spare = spare_resources.entry(input.product).or_insert(0);
                    if *spare >= input.count {
                        *spare -= input.count
                    } else {
                        todo.push((&input.product, input.count));
                    }
                }
            }
            if reaction.count > count {
                *spare_resources.entry(product).or_insert(0) += reaction.count - count;
            }
            if reaction.count < count {
                // biggest source of slowness
                todo.push((product, count - reaction.count));
            }
        }
    }
    i64::from(ore_count)
}

#[derive(Debug)]
struct Reaction<'a> {
    product: &'a str,
    count: i32,
    inputs: Vec<ReactionEntry<'a>>,
}

#[derive(Debug)]
struct ReactionEntry<'a> {
    product: &'a str,
    count: i32,
}

fn parse_reaction_entry(entry: &str) -> ReactionEntry {
    let (count, product) = entry.split_whitespace().next_tuple().unwrap();
    ReactionEntry {
        product,
        count: count.parse().unwrap(),
    }
}

fn parse_reaction(reaction: &str) -> Reaction {
    let parts: (&str, &str) = reaction.split("=>").next_tuple().unwrap();
    let output = parse_reaction_entry(parts.1);
    Reaction {
        product: output.product,
        count: output.count,
        inputs: parts.0.split(',').map(parse_reaction_entry).collect(),
    }
}

fn parse_reactions(reactions: &str) -> HashMap<&str, Reaction> {
    reactions.trim().lines().map(parse_reaction).map(|e| (e.product, e)).collect()
}

#[test]
fn test_parse() {
    let reactions = parse_reactions("
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
    ");
    assert_eq!(reactions.get("CA").unwrap().inputs.first().unwrap().count, 4);
    assert_eq!(reactions.get("CA").unwrap().inputs.first().unwrap().product, "C");
}

#[test]
fn test_example1() {
    let reactions = parse_reactions("
        10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL
    ");
    assert_eq!(build_product("A", 1, &reactions, &mut HashMap::new()), 10);
    assert_eq!(build_product("A", 11, &reactions, &mut HashMap::new()), 20);
    assert_eq!(build_product("FUEL", 1, &reactions, &mut HashMap::new()), 31);
}
