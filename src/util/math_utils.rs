use std::collections::{HashSet, HashMap};

const FACTORIAL_TABLE: HashMap<i32, i64> = HashMap::new();

pub fn multiset_permute(set: Vec<i32>) {
    // let permutations = vec![vec![0, set.len()], calculate_unique_permutations(set)];
}

fn calculate_unique_permutations(set: Vec<i32>) -> i32 {
    let multiplicities = calculate_multiplicities(set);
    let max = multiplicities.iter().max();
}


fn calculate_multiplicities(set: Vec<i32>) -> Vec<i32> {
    let map: HashMap<i32, i32> = set
        .iter()
        .fold(HashMap::new(), |mut map, ele| {
            map.entry(ele.to_owned())
                .and_modify(|v| {*v += 1})
                .or_insert(1);
            return map;
        });
    
    return map.keys().cloned().collect();
}

fn factorial(num: i32) -> i32 {
    if let (FACTORIAL_TABLE.get(num)) {

    }
    let mut result: i32 = 1;

    for i in 2 .. num {
        result *= i;
    }
    return result;
}



// #[test]
// fn test_unique_elements() {
//     let set = vec![1, 1, 2, 4];
//     let val = unique_elements(set);
//
//     assert_eq!(val, 3);
// }

// [1, 1, 2, 4] multiset 
// [(1, 2), (2, 1), (4, 1)]
// n!/(m1! * m2! ... * mn!)
// [2, 1, 1
// [1, 1, 1, 1, 0, 0, 0, 0,]
// 8! / (4! * 4!)
