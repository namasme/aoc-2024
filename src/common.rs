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
