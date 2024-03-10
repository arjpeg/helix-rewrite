use std::iter::Peekable;

pub struct Cursor<T>
where
    T: Iterator,
{
    iter: Peekable<T>,
    position: usize,
    current: Option<T::Item>,
}

impl<T: Iterator> Cursor<T> {
    pub fn new(iterator: T) -> Self {
        Self {
            iter: iterator.peekable(),
            position: 0,
            current: None,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn current(&self) -> Option<&T::Item> {
        self.current.as_ref()
    }

    pub fn peek(&mut self) -> Option<&T::Item> {
        self.iter.peek()
    }

    pub fn advance(&mut self) -> Option<&T::Item> {
        self.position += 1;
        self.current = self.iter.next();

        self.current.as_ref()
    }

    pub fn advance_while(&mut self, predicate: impl Fn(&T::Item) -> bool) -> usize {
        let start = self.position;

        while let Some(char) = self.peek() {
            if !predicate(char) {
                break;
            }

            self.advance();
        }

        self.position - start
    }
}
