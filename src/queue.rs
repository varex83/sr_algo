// Черги. Черги часто використовуються для моделювання потоку
// людей, машин, літаків, банківських операцій і т. д.
// Напишіть програму, що моделює чергу покупців до каси в магазині.
// Програма повинна відображати вміст відразу декількох черг. Новий покупець
// переміщується в чергу натисканням клавіші. Ви повинні самостійно
// визначити, яким чином він буде вибирати чергу (мінімальна довжина черги).
// Обслуговування кожного покупця має випадкову тривалість (в залежності від
// кількості товарів в кошику). Обслужені покупці видаляються з черги.

use rand::random;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Queue {
    queues: Vec<VecDeque<(String, u64)>>,
    time: u64,
}

impl Queue {
    pub fn new(queues: usize) -> Self {
        Queue {
            queues: vec![VecDeque::new(); queues],
            time: 0,
        }
    }

    pub fn push(&mut self, item: String) {
        let mut min_index = 0;
        let mut min_len = usize::MAX;
        for (i, queue) in self.queues.iter().enumerate() {
            if queue.len() < min_len {
                min_len = queue.len();
                min_index = i;
            }
        }
        let end_time = random::<u64>() % 10 + 1;
        self.queues[min_index].push_back((item.clone(), end_time));
        println!("Added {} to queue {}", item, min_index + 1);
    }

    pub fn pop(&mut self) -> Option<(String, u64)> {
        for (i, queue) in self.queues.iter_mut().enumerate() {
            match queue.front() {
                Some((element, time)) if *time == 0 => {
                    println!("{} was served and removed from queue {}", element, i + 1);
                    return queue.pop_front();
                }
                Some((_, time)) => {
                    queue.front_mut().unwrap().1 = *time - 1;
                }
                _ => {}
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.queues.iter().all(|queue| queue.is_empty())
    }

    pub fn run(&mut self) {
        let mut generator = names::Generator::default();

        for _ in 0..3 {
            self.queues.push(VecDeque::new());
        }

        loop {
            self.time += 1;
            if self.time % 3 == 0 {
                self.push(generator.next().unwrap());
            }
            if self.pop().is_some() {
                continue;
            }
            if self.is_empty() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new(5);
        queue.push("Alice".to_string());
        queue.push("Bob".to_string());
        queue.push("Charlie".to_string());
        queue.push("David".to_string());
        queue.push("Eve".to_string());
        queue.push("Frank".to_string());
        queue.push("Grace".to_string());
        queue.push("Heidi".to_string());
        queue.push("Ivan".to_string());
        queue.push("Judy".to_string());
        queue.push("Kevin".to_string());
        queue.push("Linda".to_string());
        queue.push("Michael".to_string());
        queue.push("Nancy".to_string());
        queue.push("Oliver".to_string());
        queue.push("Pamela".to_string());
        queue.push("Quentin".to_string());
        queue.push("Rachel".to_string());
        queue.push("Steve".to_string());
        queue.push("Tina".to_string());
        queue.push("Ursula".to_string());
        queue.push("Victor".to_string());
        queue.push("Wendy".to_string());
        queue.push("Xavier".to_string());
        queue.push("Yvonne".to_string());
        queue.push("Zach".to_string());
        queue.run();
    }
}
