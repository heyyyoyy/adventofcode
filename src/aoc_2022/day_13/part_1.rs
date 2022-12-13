use serde_json;
use serde::Deserialize;
use std::cmp::Ordering;



#[derive(Debug, Deserialize, Eq, PartialOrd, PartialEq)]
#[serde(untagged)]
enum Content {
    Int(i32),
    Array(Vec<Content>)
}

impl Ord for Content {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Content::Array(left_vec), Content::Array(right_vec)) => left_vec.cmp(right_vec),
            (Content::Int(left_num), Content::Int(right_num)) => left_num.cmp(right_num),
            (Content::Int(left_num), Content::Array(right_vec)) => vec![Content::Int(*left_num)].cmp(right_vec),
            (Content::Array(left_vec), Content::Int(right_num)) => left_vec.cmp(&vec![Content::Int(*right_num)]),
        }
    }
}


#[derive(Debug)]
struct Pair {
    left: Content,
    right: Content
}


fn get_pairs(input_str: &str) -> Vec<Pair> {
    input_str
    .split("\n\n")
    .map(|lines| {
        let mut pair = lines.lines();
    
        let left: Content = serde_json::from_str(pair.next().unwrap()).unwrap();
        let right: Content = serde_json::from_str(pair.next().unwrap()).unwrap();
        Pair {left, right}
    })
    .collect()
}


fn distress_signal(input_str: &str) -> usize {

    let pairs = get_pairs(input_str);

    pairs
    .iter()
    .enumerate()
    .filter_map(|(idx, Pair { left, right })| {
        match left.cmp(right) {
            Ordering::Equal => unreachable!("somes wrong"),
            Ordering::Less => Some(idx + 1),
            Ordering::Greater => None,
        }
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_distress_signal() {
        let input_str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(13, distress_signal(input_str));
    }

    #[test]
    fn test_distress_signal_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!(5350, distress_signal(input_str));
    }

    #[bench]
    fn bench_distress_signal(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            distress_signal(input_str)
        })
    }
}
