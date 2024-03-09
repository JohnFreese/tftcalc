use factorial::factorial;
use std::{collections::HashMap, io::Empty, ptr::{self, null_mut, null}};

use crate::util::linked_list::Node;

use super::linked_list::LinkedList;

use crate::ll;

// Based off this algorithim: https://epubs.siam.org/doi/pdf/10.1137/1.9781611973068.107
pub fn multiset_permute(set: Vec<usize>) -> Vec<Vec<usize>> {
    let mut permutations: Vec<Vec<usize>> = Vec::with_capacity(calculate_unique_permutations(&set));
    let mut slice = set.clone();
    let mut ll = LinkedList::from(slice);

    unsafe {    
        let mut h = ll.get_raw_reference(0); // first element
        let mut i = ll.get_raw_reference(set.len() - 2); // second to last element
        let mut j = ll.get_raw_reference(set.len() - 1); // last element
        let mut s: *mut Node<usize> = ptr::null_mut();
        let mut t: *mut Node<usize> = ptr::null_mut();

        permutations.push(Vec::from(&ll));
        while (*j).next != null_mut() || (*j).value < (*h).value {
            if (*j).next != null_mut() && (*i).value >= (*(*j).next).value {
                s = j;
            } else {
                s = i;
            }
            t = (*s).next;
            (*s).next = (*t).next;
            (*t).next = h;
            if ((*t).value < (*h).value) {
                i = t;
            }
            j = (*i).next;
            h = t;
            ll.set_head(h);

            permutations.push(Vec::from(&ll));
        }
    }

    permutations
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

#[test]
fn test_multiset_permute() {
    let vec = vec![4, 2, 1, 1];
    let actual = multiset_permute(vec);
    let expected = vec![
        vec![4, 2, 1, 1], 
        vec![1, 4, 2, 1], 
        vec![4, 1, 2, 1],
        vec![1, 4, 1, 2],
        vec![1, 1, 4, 2],
        vec![4, 1, 1, 2],
        vec![2, 4, 1, 1],
        vec![1, 2, 4, 1],
        vec![2, 1, 4, 1],
        vec![1, 2, 1, 4],
        vec![1, 1, 2, 4],
        vec![2, 1, 1, 4]
    ];

    assert_eq!(actual, expected);
}

// [1, 1, 2, 4] multiset
// [(1, 2), (2, 1), (4, 1)]
// n!/(m1! * m2! ... * mn!)
// [2, 1, 1
// [1, 1, 1, 1, 0, 0, 0, 0,]
// 8! / (4! * 4!)

#[test]
fn test_calc_uniq_perms() {
    let multis = vec![4, 2, 1, 1];

    assert_eq!(calculate_unique_permutations(&multis), 12);
}

#[test]
fn test_fac() {
    let fac_8 = 40320;
    assert_eq!(factorial(8), fac_8);
}
