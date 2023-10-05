use core::num;
use std::sync::Arc;
use std::thread;
use std::sync::Mutex;
use num_cpus;

/// Split computing of function f over collection input into threads
/// Amount of threads is length of collection divided by threshold
/// # Examples
///
/// Basic usage:
///
/// ```
/// let vector = vec![1, 2, 3];
/// let squares = rust_test::split_work(vector, |x| x * x , 2);
/// assert_eq!(squares, &[1, 4, 9]);
/// ```
/// 

pub fn split_work<T, R>(input: Vec<T>, f: fn(T) -> R, threshold: usize) -> Vec<R>
where
    T: Send + 'static + std::marker::Sync + Copy,
    R: Send + 'static + std::fmt::Debug + std::default::Default + Copy,
{
    let input_size = input.len();

    if input_size <= threshold {
        return input.into_iter().map(f).collect();
    }
    let num_threads = std::cmp::min(num_cpus::get(), input.len() + 1 / 10 + 1);
    let chunk_size = (input.len() + num_threads - 1) / num_threads;
    let input_arc = Arc::new(input.clone());
    let result_mutex = Arc::new(Mutex::new(vec![Default::default(); input_size]));
    let mut threads = Vec::with_capacity(num_threads);

    for i in 0..num_threads {
        let arc_res = Arc::clone(&result_mutex);
        let arc_data_thread = Arc::clone(&input_arc);
        threads.push(thread::spawn(move || {
            let right_border = std::cmp::min((i + 1) * chunk_size, arc_data_thread.len());
            let mut inner = vec![];
            for j in i * chunk_size..right_border {
                inner.push(f(arc_data_thread[j]));
            }
            //mutex is locking only after computations, so they remain parellelism
            let mut guard = arc_res.lock().unwrap();
            for j in i * chunk_size..right_border {
                guard[j] = inner[j - i * chunk_size];
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let lock = Arc::try_unwrap(result_mutex).expect("Rc unwrap failed!");
    lock.into_inner().expect("Mutex into_inner failed!")
}