use std::cmp::Ordering;

#[derive(Debug)]
pub struct Cycle {
    pub mu: usize,     // the length of the prefix
    pub lambda: usize, // the actual cycle length
}

pub trait CycleDetection {
    fn detect_cycle(&self) -> Option<Cycle>;
}

impl<T> CycleDetection for T
where
    T: IntoIterator + Clone,
    T::Item: Eq,
{
    fn detect_cycle(&self) -> Option<Cycle> {
        let mut tortoise_iter = self.clone().into_iter();
        let mut hare_iter = self.clone().into_iter().skip(1);
        let mut tortoise = tortoise_iter.next()?;
        let mut hare = hare_iter.next()?;
        while tortoise != hare {
            tortoise = tortoise_iter.next()?;
            hare_iter.next();
            hare = hare_iter.next()?;
        }

        let mut mu = 0;
        tortoise_iter = self.clone().into_iter(); // reset stream
        tortoise = tortoise_iter.next()?;
        hare = hare_iter.next()?;
        while tortoise != hare {
            tortoise = tortoise_iter.next()?;
            hare = hare_iter.next()?;
            mu += 1;
        }

        let mut lambda = 1;
        hare = hare_iter.next()?;
        while tortoise != hare {
            hare = hare_iter.next()?;
            lambda += 1;
        }

        Some(Cycle { mu, lambda })
    }
}

pub fn pairs<T: Clone>(list: &[T]) -> Vec<(T, T)> {
    list.iter()
        .enumerate()
        .flat_map(|(idx, a)| list.iter().skip(idx + 1).map(|b| (a.clone(), b.clone())))
        .collect()
}

/// Calculates r, s, t such that s * a + t * b = r = gcd(a, b)
pub fn bezout(a: usize, b: usize) -> (usize, isize, isize) {
    // Taken from https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
    let (mut r_0, mut r_1) = (a, b);
    let (mut s_0, mut s_1) = (1, 0);
    let (mut t_0, mut t_1) = (0, 1);

    while r_1 != 0 {
        let q = (r_0 / r_1) as isize;
        let r = r_0 % r_1;
        let s = s_0 - q * s_1;
        let t = t_0 - q * t_1;

        (r_0, r_1) = (r_1, r);
        (s_0, s_1) = (s_1, s);
        (t_0, t_1) = (t_1, t);
    }

    (r_0, s_0, t_0)
}

/// Binary search for the first index where the predicate changes sign.
pub fn binary_search<P: Fn(usize) -> Ordering>(left: usize, right: usize, predicate: P) -> Match {
    let mut low = left;
    let mut high = right;
    let left_sign = predicate(low);
    let right_sign = predicate(high);

    if left_sign == Ordering::Equal {
        return Match::Exact(low);
    } else if right_sign == Ordering::Equal {
        return Match::Exact(high);
    } else if left_sign == right_sign {
        return Match::Nothing;
    }

    while low < high {
        let mid = (low + high) / 2;
        let sign = predicate(mid);
        if sign == left_sign {
            low = mid + 1;
        } else if sign == right_sign {
            high = mid;
        } else {
            return Match::Exact(mid);
        }
    }

    Match::After(low)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Match {
    Exact(usize),
    After(usize),
    Nothing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cycle() {
        // 0 1 2 3 4 (5 6 7 8 9 10) (5 6 7 8 9 10) ...
        let iterator = (0..5).chain((5..11).cycle());
        let result = iterator.detect_cycle();
        assert!(result.is_some());

        let cycle = result.unwrap();
        assert_eq!(cycle.mu, 5);
        assert_eq!(cycle.lambda, 6);
    }

    #[test]
    fn test_pairs() {
        let list = vec![1, 2, 3, 4];
        let pairs = pairs(&list);
        assert_eq!(pairs, vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]);
    }

    #[test]
    fn test_bezout() {
        let (r, s, t) = bezout(240, 46);
        assert_eq!(r, 2);
        assert_eq!(s, -9);
        assert_eq!(t, 47);
    }

    #[test]
    fn test_binary_search_first_sign_change() {
        let cases = [
            (vec![-5, -3, -2, -1, 2, 4, 6], Match::After(4)),
            (vec![-5, -3, -1, 2, 4, 6], Match::After(3)),
            (vec![-1, 2, 4, 6], Match::After(1)),
            (vec![-5, -3, -2, -1, 2], Match::After(4)),
            (vec![-5, -3, -2, -1, 0, 2, 4, 6], Match::Exact(4)),
            (
                vec![-5, -3, -2, -1, 2, 4, 6].into_iter().rev().collect(),
                Match::After(3),
            ),
            (vec![1, 2, 3], Match::Nothing),
            (vec![-3, -2, -1], Match::Nothing),
            (vec![-3, -2, -1, 0], Match::Exact(3)),
            (vec![0, 1, 2, 3], Match::Exact(0)),
        ];

        for (xs, expected) in cases {
            assert_eq!(expected, binary_search(0, xs.len() - 1, |i| xs[i].cmp(&0)));
        }
    }
}
