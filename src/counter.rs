extern crate num;

use self::num::integer::Integer;

use metric::Metric;

pub struct StdCounter<T: Integer> {
    pub value: T,
}

pub trait Counter<T: Integer>: Metric {
    fn clear(&mut self);

    fn dec(&mut self, value: T);

    fn inc(&mut self, value: T);

    fn snapshot(self) -> Self;
}

impl<T: Integer> Counter<T> for StdCounter<T> {
    fn clear(&mut self) {
        self.value = Integer::zero();
    }

    fn dec(&mut self, value: T) {
        self.value = self.value - value;
    }

    fn inc(&mut self, value: T) {
        self.value = self.value + value;
    }

    fn snapshot(self) -> StdCounter<T> {
        StdCounter { value: self.value }
    }
}

impl<T: Integer> Metric for StdCounter<T> { }

impl<T: Integer> StdCounter<T> {
    pub fn new() -> StdCounter<T> {
        StdCounter{ value: Integer::zero() }
    }
}

#[cfg(test)]
mod test {
    use counter::StdCounter;
    use counter::Counter;

    #[test]
    fn increment_by_1() {
        let mut c: StdCounter<Integer> = StdCounter{ value: 0i64 };
        c.inc(1);

        assert!(c.value == 1);
    }

    #[test]
    fn snapshot() {
        let c: StdCounter<Integer> = StdCounter{value: 0i64 };
        let mut c_snapshot = c.snapshot();

        c_snapshot.inc(1);

        assert!(c.value == 0);
        assert!(c_snapshot.value == 1);
    }
}
