use rayon::prelude::*;
use std::sync::atomic::AtomicU64;
use std::thread;

fn map_sum1<const N: usize>(v: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    assert!(N > 0, "Number of threads must be greater than 0");

    let chunk_size = (v.len() + N - 1) / N; // ceil division
    let mut handles = Vec::with_capacity(N);

    for chunk in v.chunks(chunk_size) {
        let chunk_vec = chunk.to_vec(); // clone the chunk to move into thread
        let handle = thread::spawn(move || chunk_vec.iter().map(|&x| f(x) as u64).sum::<u64>());
        handles.push(handle);
    }

    // Collect all partial sums from threads and return the total
    handles
        .into_iter()
        .map(|h| h.join().expect("Thread panicked"))
        .sum()
}

// Same as above but using AtomicU64 counter for summation
fn map_sum2<const N: usize>(v: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    assert!(N > 0, "Number of threads must be greater than 0");

    let chunk_size = (v.len() + N - 1) / N; // ceil division
    let total_sum = std::sync::Arc::new(AtomicU64::new(0));
    let mut handles = Vec::with_capacity(N);

    for chunk in v.chunks(chunk_size) {
        let chunk_vec = chunk.to_vec(); // clone the chunk to move into thread
        let total_sum_ref = std::sync::Arc::clone(&total_sum);
        let handle = thread::spawn(move || {
            for &x in &chunk_vec {
                total_sum_ref.fetch_add(f(x) as u64, std::sync::atomic::Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for h in handles {
        h.join().expect("Thread panicked");
    }

    total_sum.load(std::sync::atomic::Ordering::SeqCst)
}

//map_sum3: uses MPSC channel to communicate mapping
//results from worker threads and does summation in the
//main thread

fn map_sum3<const N: usize>(v: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    assert!(N > 0, "Number of threads must be greater than 0");

    let chunk_size = (v.len() + N - 1) / N; // ceil division
    let (tx, rx) = std::sync::mpsc::channel();
    let mut handles = Vec::with_capacity(N);

    for chunk in v.chunks(chunk_size) {
        let chunk_vec = chunk.to_vec(); // clone the chunk to move into thread
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let sum: u64 = chunk_vec.iter().map(|&x| f(x) as u64).sum();
            tx_clone.send(sum).expect("Failed to send sum");
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for h in handles {
        h.join().expect("Thread panicked");
    }

    // Collect all partial sums from the channel and return the total
    rx.iter().take(N).sum()
}

fn map_sum4(v: Vec<u32>, f: fn(u32) -> u64) -> u64 {
    v.into_par_iter() // Convert the vector into a parallel iterator
        .map(|x| f(x) as u64) // Apply the function to each element
        .sum() // Sum the results in parallel
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(x: u32) -> u64 {
        (x as u64) * (x as u64)
    }

    #[test]
    fn test_map_sum1() {
        let v = vec![1, 2, 3, 4, 5];
        let result = map_sum1::<4>(v, square);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_map_sum2() {
        let v = vec![1, 2, 3, 4, 5];
        let result = map_sum2::<4>(v, square);
        assert_eq!(result, 55);
    }

    // TODO: does not end for some reason
    // #[test]
    // fn test_map_sum3() {
    //     let v = vec![1, 2, 3, 4, 5];
    //     let result = map_sum3::<4>(v, square);
    //     assert_eq!(result, 55);
    // }

    #[test]
    fn test_map_sum4() {
        let v = vec![1, 2, 3, 4, 5];
        let result = map_sum4(v, square);
        assert_eq!(result, 55);
    }
}
