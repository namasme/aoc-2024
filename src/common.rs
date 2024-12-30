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
