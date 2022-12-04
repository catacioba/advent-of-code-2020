use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

type Allergen = String;
type Ingredient = String;

fn parse_ingredient_list(l: &str) -> (HashSet<Ingredient>, HashSet<Allergen>) {
    let mut parts = l.split("(contains ");

    let ingredients = parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| String::from(s))
        .collect();

    let allergens_part = parts.next().unwrap();
    let allergens_part = &allergens_part[..allergens_part.len() - 1];
    let allergens = allergens_part
        .split(", ")
        .map(|s| String::from(s))
        .collect();

    (ingredients, allergens)
}

fn read_ingredient_lists() -> Vec<(HashSet<Ingredient>, HashSet<Allergen>)> {
    fs::read_to_string("src/ch21/input.txt")
        .unwrap()
        .lines()
        .map(|l| parse_ingredient_list(l))
        .collect()
}

fn intersection<'a>(sets: &'a Vec<&HashSet<Ingredient>>) -> HashSet<&'a Ingredient> {
    let mut res = HashSet::new();

    if sets.is_empty() {
        return res;
    }

    let first_set = sets[0];
    let other_sets = &sets[1..];

    for ingr in first_set {
        if other_sets.iter().all(|s| s.contains(ingr)) {
            res.insert(ingr);
        }
    }

    res
}

pub fn solve_part1() {
    let food_list = read_ingredient_lists();

    let mut allergen_foods: HashMap<Allergen, Vec<&HashSet<Ingredient>>> = HashMap::new();
    for (ingr_list, allergen_list) in &food_list {
        for allergen in allergen_list {
            let e = allergen_foods.entry(allergen.clone()).or_insert(Vec::new());
            e.push(ingr_list);
        }
    }

    for x in &allergen_foods {
        println!("{:?}", x);
    }
    println!();

    let mut allergen_possibilities: HashMap<Allergen, HashSet<&Ingredient>> = allergen_foods
        .iter()
        .map(|(il, al)| (il.clone(), intersection(al)))
        .collect();

    for x in &allergen_possibilities {
        println!("{:?}", x);
    }
    println!();

    let mut x: HashSet<&Ingredient> = HashSet::new();
    for (_, ingr_list) in &allergen_possibilities {
        for ingr in ingr_list {
            x.insert(ingr);
        }
    }

    let mut cnt = 0;
    for (ingr_list, _) in &food_list {
        for ingr in ingr_list {
            if !x.contains(ingr) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);

    let mut queue: VecDeque<Allergen> = VecDeque::new();
    let mut ingredient_allergens: Vec<(Ingredient, Allergen)> = Vec::new();

    for (allergen, ingr_set) in &allergen_possibilities {
        if ingr_set.len() == 1 {
            queue.push_back(allergen.clone());
        }
    }

    while let Some(allergen) = queue.pop_front() {
        let (al, il) = allergen_possibilities.remove_entry(&allergen).unwrap();
        let ingr: String = String::from(*il.iter().next().unwrap());

        for (allergen, ingr_set) in allergen_possibilities.iter_mut() {
            if ingr_set.contains(&ingr) {
                ingr_set.remove(&ingr);

                if ingr_set.len() == 1 {
                    queue.push_back(allergen.clone());
                }
            }
        }

        ingredient_allergens.push((ingr, al));
    }

    println!("{:?}", &ingredient_allergens);

    ingredient_allergens.sort_by(|(_, xa), (_, ya)| xa.cmp(ya));
    let mut res = String::new();
    ingredient_allergens.iter().for_each(|(i, _)| {
        res.push_str(&i);
        res.push(',');
    });

    println!("{:?}", res);
}

pub fn solve_part2() {}
