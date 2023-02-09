pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn shift(&mut self) -> Option<char> {
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

impl Queue {
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

impl Queue {
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
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
}
