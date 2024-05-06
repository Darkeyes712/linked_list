#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    pub list: Vec<T>,
}

impl<T: std::fmt::Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        if self.list.is_empty() {
            return true;
        } else {
            return false;
        }
    }

    pub fn len(&self) -> usize {
        self.list.len() as usize
    }

    pub fn push(&mut self, element: T) {
        self.list.push(element)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.list.is_empty() {
            None
        } else {
            self.list.pop()
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.last()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        Self {
            list: self.list.into_iter().map(|x| x).rev().collect(),
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            list: iter.into_iter().collect(),
        }
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        linked_list.list.into_iter().collect::<Vec<_>>()
    }
}
