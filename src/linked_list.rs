struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>, // Some(Box::new(T)), None,
}

impl<T> Node<T> {
    fn len(&self) -> usize {
        match &self.next {
            None => 1,
            Some(node) => 1 + node.len(),
        }
    }

    fn add(&mut self, value: T) {
        match &mut self.next {
            Some(node) => node.add(value),
            None => self.next = Some(Box::new(Node { value, next: None })),
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index == 0{
            Some(&self.value)
        } else {
            match &self.next {
                None => None,
                Some(next) => next.get(index - 1),
            }
        }
    }

    fn insert(&mut self, index: usize, value: T) {
        // match (index, &mut self.next) {
        //     (0, _) => Box::,
        // }


    }

}




