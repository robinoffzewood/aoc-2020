use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut tst_food = FoodList::from_file("example.txt");
    let mut food = FoodList::from_file("input.txt");

    // part 1
    let tst_inerts = tst_food.inert_ingredients();
    assert_eq!(5, tst_food.count_inert(&tst_inerts));
    let inerts = food.inert_ingredients();
    println!(
        "Ingredients without allergens appears {} times",
        food.count_inert(&inerts)
    );

    // part2
    let tst_dangerous_list = tst_food.dangerous_list();
    assert_eq!("mxmxvkd,sqjhc,fvjkl", tst_dangerous_list);
    let dangerous_list = food.dangerous_list();
    println!("Canonical dangerous ingredient list: {}", dangerous_list);

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}
struct FoodList {
    // List of food with ingredients, and possible allergens
    food: Vec<Food>,
    // Correspondence between each allergen and container ingredients
    allergens_table: HashMap<String, HashSet<String>>,
}

impl FoodList {
    fn inert_ingredients(&mut self) -> HashSet<String> {
        // Make a list the ingredients that are not in any list of possible container of allergens
        let mut inert_list: HashSet<String> = HashSet::new();

        // for each food in the list
        //  for each allergens in this food
        //    compute the intersection of the new set of ingredients possibly containing this allergen
        //     with existing set. Replace the ingredients list by the intersected one.
        let mut all_ingredients: HashSet<String> = HashSet::new();
        for f in &self.food {
            for a in &f.allergens {
                let mut new_set = HashSet::from_iter(f.ingredients.iter().cloned());
                if let Some(existing_set) = self.allergens_table.get(a) {
                    new_set = existing_set.intersection(&new_set).cloned().collect();
                }
                self.allergens_table.insert(a.clone(), new_set);
                //println!("allergens {} could be in {:?}", a, self.allergens_table.get(a));

                for i in &f.ingredients {
                    all_ingredients.insert(i.clone());
                }
            }
        }
        'ingredient: for i in &all_ingredients {
            for a in &self.allergens_table {
                if a.1.contains(i) {
                    continue 'ingredient;
                }
            }
            inert_list.insert(i.clone());
        }
        inert_list
    }

    fn count_inert(&self, inert_list: &HashSet<String>) -> usize {
        let mut inert_cnt = 0;
        for f in &self.food {
            for i in &f.ingredients {
                if inert_list.contains(i) {
                    inert_cnt += 1;
                }
            }
        }
        inert_cnt
    }

    fn dangerous_list(&mut self) -> String {
        // Loop on every allergens, find the first that has only one ingredient possibly containing it
        //   and remove this ingredient from the list of ingredients of all other allergens
        // make a list of all allergens to visit
        let mut allergen_list: HashSet<String> = self.allergens_table.keys().cloned().collect();
        'allergen_loop: loop {
            if allergen_list.len() == 0 {
                break;
            }
            let mut allergen_name = "".to_string();
            let mut ingredient_to_reduce = "".to_string();
            'most_constrained: for a in &mut self.allergens_table {
                if a.1.len() == 1 && allergen_list.contains(a.0) {
                    allergen_name = a.0.clone();
                    ingredient_to_reduce = a.1.iter().next().unwrap().clone();
                    break 'most_constrained;
                }
            }
            if allergen_name.is_empty() {
                break 'allergen_loop;
            }
            'reduce: for a in &mut self.allergens_table {
                if a.0 == &allergen_name {
                    continue 'reduce;
                }
                a.1.remove(&ingredient_to_reduce);
            }
            allergen_list.remove(&allergen_name);
        }
        // Sort by allergen name
        let mut canonical_list = "".to_string();
        let mut allergen_sorted: Vec<String> = self
            .allergens_table
            .keys()
            .cloned()
            .collect::<Vec<String>>();
        allergen_sorted.sort();
        for dangerous in allergen_sorted {
            let ingr = self
                .allergens_table
                .get(&dangerous)
                .unwrap()
                .iter()
                .next()
                .unwrap();
            canonical_list += ingr;
            canonical_list += ",";
        }
        canonical_list.strip_suffix(",").unwrap().to_string()
    }

    fn from_file(f_name: &str) -> Self {
        let mut food = Vec::new();
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        let food_list: Vec<&str> = str_in.split("\r\n").collect();
        for f in food_list {
            // Split the line in 2, at "(contains"
            let ingr_allergens: Vec<&str> = f.split("(contains ").collect();
            // Gather ingredients list
            let ingredients = ingr_allergens[0]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let allergens = ingr_allergens[1]
                .strip_suffix(")")
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            food.push(Food {
                ingredients,
                allergens,
            });
        }
        FoodList {
            food,
            allergens_table: HashMap::new(),
        }
    }
}
