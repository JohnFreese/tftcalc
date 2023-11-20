use factorial::factorial;
use std::{collections::HashMap, io::Empty};

use crate::util::linked_list::Node;

use super::linked_list::LinkedList;

// [1, 1, 2, 4] multiset
pub fn multiset_permute(set: Vec<usize>) {
    // let permutations = vec![vec![0, set.len()], calculate_unique_permutations(set)];
    let mut permutations: Vec<Vec<usize>> = Vec::with_capacity(calculate_unique_permutations(&set));
    let mut slice = set.clone();
    slice.reverse();
    let mut ll = LinkedList::from(slice);

    type Link<'a> = Option<&'a mut Node<usize>>;
    let h: Link = ll.get_mut(0); // first element
    let i: Link = ll.get_mut(set.len() - 2); // second to last element
    let j: Link = ll.get_mut(set.len() - 1); // last element
    let s: Link = None;
    let t: Link = None;

    // loopless iteration
    // while let Some(jnext) = j.map(|n| n.next) {
    while (j.map(|n| n.next).is_some() || j.map(|n| n.value) < h.map(|n| n.value)) {
        let single_perm = ll.clone();

        permutations.push(Vec::from(single_perm));
    }
}

// n!/(m1! * m2! ... * mn!)
fn calculate_unique_permutations(set: &Vec<usize>) -> usize {
    let multiplicities = calculate_multiplicities(&set);

    let multi_facs = multiplicities.iter().fold(1 as usize, |a, e| {
        return a.to_owned() * factorial(e.to_owned());
    });

    return factorial(set.len()) / multi_facs;
}

fn calculate_multiplicities(set: &Vec<usize>) -> Vec<usize> {
    let map: HashMap<usize, usize> = set.iter().fold(HashMap::new(), |mut map, ele| {
        map.entry(ele.to_owned())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        return map;
    });

    return map.values().cloned().collect();
}

mod factorial {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    const FACTORIAL_TABLE: Lazy<HashMap<usize, usize>> = Lazy::new(|| HashMap::new());

    pub fn factorial(num: usize) -> usize {
        if num < 2 {
            return 1;
        }
        if let Some(fac) = FACTORIAL_TABLE.get(&num) {
            return fac.to_owned();
        }

        let res = num * factorial(num - 1);
        FACTORIAL_TABLE.insert(num, res);

        return res;
    }
}

// [1, 1, 2, 4] multiset
// [(1, 2), (2, 1), (4, 1)]
// n!/(m1! * m2! ... * mn!)
// [2, 1, 1
// [1, 1, 1, 1, 0, 0, 0, 0,]
// 8! / (4! * 4!)

#[test]
fn test_calc_uniq_perms() {
    let multis = vec![1, 1, 2, 4];

    // assert_eq!(calculate_unique_permutations(multis), 12);
}

#[test]
fn test_fac() {
    let fac_8 = 40320;
    assert_eq!(factorial(8), fac_8);
}
