use std::collections::HashMap;

use metric::Metric;

pub trait Registry<'a> {
    fn get(&'a self, name: &'a str) -> &'a Metric;

    fn insert<T: Metric + 'a>(&mut self, name: &'a str, metric: T);
}

pub struct StdRegistry<'a> {
    metrics: HashMap<&'a str, Box<Metric + 'a>>,
}

// Specific stuff for registry goes here
impl<'a> Registry<'a> for StdRegistry<'a> {
    fn get(&'a self, name: &'a str) -> &'a Metric {
        &*self.metrics[name]
    }

    fn insert<T: Metric + 'a>(&mut self, name: &'a str, metric: T) {
        let boxed: Box<Metric> = Box::New();

        self.metrics.insert(name, boxed);
    }
}

// General StdRegistry
impl<'a> StdRegistry<'a> {
    fn new() -> StdRegistry<'a> {
        StdRegistry{
            metrics: HashMap::new()
        }
    }
}

#[cfg(test)]
mod test {
    use metric::Metric;
    use meter::StdMeter;
    use registry::{Registry, StdRegistry};

    #[test]
    fn meter() {
        let mut r: StdRegistry = StdRegistry::new();
        let mut m: StdMeter = StdMeter::new();

        r.insert("foo", m);
        r.get("foo");
    }
}
