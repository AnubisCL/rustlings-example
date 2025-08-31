use std::{sync::mpsc, thread, time::Duration};
use std::sync::{Arc, RwLock};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    let rw_q = Arc::new(RwLock::new(q));
    let tx = Arc::new(RwLock::new(tx));
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    let mut handles = Vec::new();
    
    
    // into the first thread. How could you solve this problem?
    let q1 = Arc::clone(&rw_q);
    let tx1 = Arc::clone(&tx);
    let t1 = thread::spawn(move || {
        let qq1 = q1.read().unwrap();
        let rw_tx_m = tx1.write().unwrap();
        for val in &qq1.first_half {
            println!("Sending {val:?}");
            rw_tx_m.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
    handles.push(t1);
    let q2 = Arc::clone(&rw_q);
    let tx2 = Arc::clone(&tx);
    let t2 = thread::spawn(move || {
        let qq2 = q2.read().unwrap();
        let rw_tx_m = tx2.write().unwrap();
        for val in &qq2.second_half {
            println!("Sending {val:?}");
            rw_tx_m.send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
    handles.push(t2);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
