use std::{
    thread,
    time::{Duration, Instant},
    sync::mpsc,
    convert::TryInto,
};

/// Sleep for the given number of nanoseconds and send the number to the given sender.
/// 
/// # Arguments
/// 
/// * `number` - The number of nanoseconds to sleep for.
/// * `sender` - The sender to send the number to.
/// 
fn eepy_worker(number: i64, sender: mpsc::Sender<Option<i64>>) {
    let start_time = Instant::now();
    let sleep_duration = Duration::from_nanos(number.try_into().expect("number cast failed"));

    // Sleep until the desired duration has passed
    thread::sleep(sleep_duration);

    // Calculate the actual elapsed time and adjust if necessary
    let elapsed_time = start_time.elapsed();
    if elapsed_time < sleep_duration {
        thread::sleep(sleep_duration - elapsed_time);
    }

    sender.send(Some(number)).expect("Error sending result");
}

/// Collect results from the given receiver and return them as a vector.
/// 
/// # Arguments
/// 
/// * `recv` - The receiver to receive results from.
///
/// # Returns
/// 
/// A vector containing all the results received from the given receiver.
fn collect_results(recv: mpsc::Receiver<Option<i64>>) -> Vec<i64> {
    let mut results = vec![];

    while let Ok(r) = recv.recv() {
        match r {
            Some(num) => results.push(num),
            None => break, // Stop collecting results when None is received
        }
    }

    results
}

/// Sort the given numbers using the sleep sort algorithm.
/// 
/// # Arguments
/// 
/// * `numbers` - The numbers to sort.
/// 
/// # Returns
/// 
/// A vector containing the sorted numbers.
/// 
/// # Example
/// 
/// ```
/// use eepsort::eepy_sort;
/// 
/// fn main() {
///    let numbers: Vec<i64> = vec![1, 6, 10, 273, 562, 1269, 1236, 1237, 1471, 12783];
///    println!("{:?}", eepy_sort(numbers));
/// }
/// ```
/// 
/// # Panics
/// 
/// Panics if the number of nanoseconds to sleep for is negative.
pub fn eepy_sort(numbers: Vec<i64>) -> Vec<i64> {
    let (tx, rx) = mpsc::channel();
    let mut eepy_workers = vec![];

    for i in numbers {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || eepy_worker(i, tx_clone));
        eepy_workers.push(handle);
    }

    // Drop the sender to signal that no more values will be sent
    drop(tx);

    // Wait for all threads to finish
    for handle in eepy_workers {
        handle.join().expect("Error joining thread");
    }

    // Collect the results
    let results = collect_results(rx);

    results
}
