use std::collections::VecDeque;

pub trait SlidingMedian {
    type Iter: Iterator;

    fn sliding_median(self, window_size: usize, max_value: usize)
        -> SlidingMedianState<Self::Iter>;
}

impl<'a, T> SlidingMedian for T
where
    T: Iterator<Item = &'a usize>,
{
    type Iter = T;

    fn sliding_median(
        self,
        window_size: usize,
        max_value: usize,
    ) -> SlidingMedianState<Self::Iter> {
        SlidingMedianState::new(self, window_size, max_value)
    }
}

pub struct SlidingMedianState<T: Iterator> {
    iter: T,
    queue: VecDeque<usize>,
    counter: Vec<usize>,
    window_size: usize,
}

impl<'a, T> SlidingMedianState<T>
where
    T: Iterator<Item = &'a usize>,
{
    fn new(iter: T, window_size: usize, max_value: usize) -> Self {
        let mut counter = Vec::with_capacity(max_value + 1);
        counter.resize(max_value + 1, Default::default());

        let queue = VecDeque::with_capacity(window_size);

        SlidingMedianState {
            iter,
            queue,
            counter,
            window_size,
        }
    }

    fn invariant(&self) -> bool {
        self.counter.iter().sum::<usize>() == self.queue.len()
    }

    fn calc_median(&self) -> f64 {
        debug_assert!(self.invariant());
        if self.queue.is_empty() {
            return 0.0;
        }

        if self.queue.len() == 1 {
            return *self.queue.front().unwrap() as f64;
        }

        let items_to_count = self.queue.len() / 2;
        let mut items_counted = 0;
        let mut break_item = None;
        let mut previous_item = None;
        let iter = self
            .counter
            .iter()
            .enumerate()
            .filter(|(_, count)| **count > 0);

        for (value, &count) in iter {
            items_counted += count;

            if count > 0 {
                if items_counted > items_to_count {
                    break_item = Some((value, count));
                    break;
                }

                previous_item = Some((value, count));
            }
        }

        let (break_value, break_count) = break_item.unwrap();
        if self.queue.len() % 2 == 0 && items_counted - break_count == items_to_count {
            let (previous_value, _) = previous_item.unwrap();
            return (break_value + previous_value) as f64 / 2.0;
        }

        break_value as f64
    }
}

impl<'a, T: Iterator<Item = &'a usize>> Iterator for SlidingMedianState<T> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.queue.is_empty() {
            let removed_val = self.queue.pop_front().unwrap();
            self.counter[removed_val] -= 1;
        };

        while self.queue.len() < self.window_size {
            let next_val = match self.iter.next() {
                Some(next_val) => next_val,
                None => return None,
            };

            self.counter[*next_val] += 1;
            self.queue.push_back(*next_val);
        }

        Some(self.calc_median())
    }
}
