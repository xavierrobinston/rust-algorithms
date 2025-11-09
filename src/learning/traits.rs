pub trait Queue {
    type Item;
    fn queue(&mut self, item: Self::Item);
    fn dequeue(&self) -> Self::Item;
}

pub struct LinkedList {
    value: i32,
}

impl Queue for LinkedList {
    type Item = i32;

    fn queue(&mut self, value: i32) {
        self.value = value;
    }
    fn dequeue(&self) -> i32 {
        self.value
    }
}

pub fn test_traits() {
    let list = &mut LinkedList { value: 10 };
    let item: i32 = 10;
    println!("Queueing item {}", item);
    list.queue(item);
    let item = list.dequeue();
    println!("Dequeueing item {}", item);
}
