use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

//mod engine;
fn main() {
    let mut segment_a: VecDeque<Arc<u32>> = VecDeque::new();
    let mut segment_b: VecDeque<Arc<u32>> = VecDeque::new();
    let mut segment_c: VecDeque<Arc<u32>> = VecDeque::new();

    println!("Before Append SegmentA: {:?}", segment_a);        // SegmentA []
    println!("Before Append + Push SegmentB: {:?}", segment_b); // SegmentB []
    println!("Before Append + Push SegmentC: {:?}", segment_c); // SegmentC []

    println!("\nPushing new data inside SegmentB and SegmentC\n");
    segment_b.push_back(Arc::new(1));
    segment_c.push_back(Arc::new(2));

    println!("Before Append + After Push SegmentB: {:?}", segment_b); // SegmentB [1]
    println!("Before Append + After Push SegmentC: {:?}", segment_c); // SegmentC [2]

    println!("\nAppending SegmentB & SegmentC to SegmentA\n");
    segment_a.append(&mut segment_b);
    segment_a.append(&mut segment_c);

    // After being appended to another VecDeque, the previous VecDecque lose its value. It doesn't clone it.
    println!("After Append SegmentA: {:?}", segment_a); // SegmentA [1, 2]
    println!("After Append SegmentB: {:?}", segment_b); // SegmentB [] // Data lost, it has been moved into SegmentA
    println!("After Append SegmentC: {:?}", segment_c); // SegmentC [] // Data lost, it has been moved into SegmentA

    // Warping SegmentA around a Mutex which is around an Arc
    let data2: Arc<Mutex<VecDeque<Arc<u32>>>> = Arc::new(Mutex::new(segment_a));

    // This will create 4 new thread that'll append value from 4-7 to the SegmentA VecDeque
    // Tx: Sender, Rx: Receiver
    let (tx_thread1,rx_thread1) = channel();

    // Loop for each thread. We'll be using the index to progressively append new values
    // We create an Arc clone from our warped data (data2), we also clone our Sender (Tx)
    let data_thread1 = Arc::clone(&data2);
    let data_thread2 = Arc::clone(&data2);


    // Thread 1:
    thread::spawn(move || {
        // Locking access to our data, blocking other thread
        let saved = {
            let queue = data_thread1.lock().unwrap();
            Arc::clone(queue.front().unwrap())
        }; // The guard is dropped here, releasing the lock.
        tx_thread1.send(saved).unwrap();
    });
    // Receive `saved` from Thread 1
    let saved = rx_thread1.recv().unwrap();



    let (tx_thread2, rx_thread2) = channel();
    // Thread 2
    thread::spawn(move || {
        let mut queue = data_thread2.lock().unwrap();
        queue.pop_front();
        queue.push_back(Arc::new(4));
        tx_thread2.send(()).unwrap();
    });

    // Receive Thread 2
    rx_thread2.recv().unwrap();
    println!("Saved: {:?}", saved);
    // Printing the final result. We're using "*" as "data2" is points to "data", we print the variable that data2's pointer points to
    println!("Final queue: {:?}", *data2.lock().unwrap());
}