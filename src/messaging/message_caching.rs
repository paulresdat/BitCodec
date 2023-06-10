use std::{collections::{HashMap}, hash::Hash};

pub struct Cache<F, Args, Ret> {
    callable: F,
    cache: HashMap<Args, Ret>
}

impl <F, Args, Ret> Cache<F, Args, Ret>
where
    F: FnMut(Args) -> Ret,
    Args: Clone + Hash + Eq,
    Ret: Clone,
{
    pub fn new(callable: F) -> Self {
        Cache { callable, cache: HashMap::new() }
    }

    pub fn call(&mut self, args: Args) -> Ret {
        if let Some(cached) = self.cache.get(&args) {
            return cached.clone();
        }

        let result = (self.callable)(args.clone());
        self.cache.insert(args, result.clone());
        result
    }
}
