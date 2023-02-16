pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    pub fn shift(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
}

impl<T> Queue<T> {
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

impl<T> Queue<T> {
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self { older: Vec::new(), younger: Vec::new() }
    }
}

fn main() {
    let mut q = Queue { older: Vec::new(), younger: Vec::new() };

    q.push('0');
    q.push('1');
    assert_eq!(q.shift(), Some('0'));


    q.push('∞');
    assert_eq!(q.shift(), Some('1'));
    assert_eq!(q.shift(), Some('∞'));
    assert_eq!(q.shift(), None);

    assert!(q.is_empty());
    q.push('Ξ');
    assert!(!q.is_empty());

    q.push('η');
    assert_eq!(q.shift(), Some('Ξ'));
    q.push('σ');

    let (older, younger) = q.split();
    assert_eq!(older, vec!['η']);
    assert_eq!(younger, vec!['σ']);

    let mut bq = Box::new(Queue::<char>::new());

    bq.push('ι');

    let mut s = Queue::new();
    let mut f = Queue::new();

    s.push("CAD");
    f.push(0.74);

    s.push("BTC");
    f.push(13764.0);
}
