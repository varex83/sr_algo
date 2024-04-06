use sr_algo::queue::Queue;

fn main() {
    let mut queue = Queue::new(5);

    let mut generator = names::Generator::default();

    for _ in 0..100 {
        queue.push(generator.next().unwrap());
    }

    queue.run()
}
