use std::{str::FromStr, iter::Sum};

#[derive(Debug)]
struct Snafu {
    number: i64
}

impl FromStr for Snafu {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let snafu_num = s.chars().map(|ch| {
            match ch {
                '2' => {2},
                '1' => {1},
                '0' => {0},
                '-' => {-1},
                '=' => {-2},
                _ => unreachable!("wrong char")
            }
        })
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, num)| {
            5i64.pow(idx as u32) * num + acc
        });
        Ok(Self {number: snafu_num})
    }
}

impl Snafu {
    fn to_snafu_number(&self) -> String {

        let mut number = self.number;
        let mut snafu_num = String::new();

        while number != 0 {
            match number % 5 {
                0 => {
                    number /= 5;
                    snafu_num.push('0');
                },
                1 => {
                    number = number - 1;
                    number /= 5;
                    snafu_num.push('1');
                },
                2 => {
                    number = number - 2;
                    number /= 5;
                    snafu_num.push('2');
                }
                3 => {
                    number = number + 2;
                    number /= 5;
                    snafu_num.push('=');
                },
                4 => {
                    number = number + 1;
                    number /= 5;
                    snafu_num.push('-');
                },
                _ => {unreachable!("wrong number")}
            }
        }
        snafu_num.chars().rev().collect()
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Snafu { number: iter.map(|s| s.number).sum() }
    }
}

impl Sum<Snafu> for i64 {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        iter.map(|s| s.number).sum()
    }
}


fn full_of_hot_air(input_str: &str) -> String {

    let snafu_num = input_str.lines().map(|line| line.parse::<Snafu>().unwrap()).sum::<Snafu>();
    snafu_num.to_snafu_number()
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_full_of_hot_air() {
        let input_str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        assert_eq!("2=-1=0", full_of_hot_air(input_str));
    }

    #[test]
    fn test_full_of_hot_air_from_file() {
        let input_str = include_str!("input.txt");
        assert_eq!("2=0=02-0----2-=02-10", full_of_hot_air(input_str));
    }

    #[bench]
    fn bench_full_of_hot_air(b: &mut Bencher) {
        let input_str = include_str!("input.txt");
        b.iter(|| {
            full_of_hot_air(input_str)
        })
    }
}
