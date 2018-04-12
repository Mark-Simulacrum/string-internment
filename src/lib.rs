use std::collections::HashSet;
use std::ops::Deref;

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub struct Interner {
    names: HashSet<&'static str>,
}

impl Interner {
    pub fn new() -> Interner {
        Interner {
            names: HashSet::new(),
        }
    }

    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&name) = self.names.get(string) {
            return Symbol(name);
        }

        let string: Box<str> = string.to_string().into_boxed_str();
        // This is safe as the str is simply leaked; since it would live until program end either
        // way, this isn't worse from a memory usage perspective.
        let s: &'static str = unsafe { &*(Box::into_raw(string) as *const str) };
        self.names.insert(s);
        Symbol(s)
    }
}

impl Deref for Symbol {
    type Target = str;
    fn deref(&self) -> &str {
        self.0
    }
}

#[test]
fn interner_1() {
    let mut i = Interner::new();
    i.intern("a");
    i.intern("a");
    assert_eq!(i.names.len(), 1);
}
