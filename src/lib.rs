pub fn parallel_computing<T: Send + 'static, R: Send + 'static>(
    items: Vec<T>,
    proc: fn(T) -> R,
) -> Vec<R> {
    let mut result = Vec::<R>::new();
    let threshold = 10usize;

    if items.len() > threshold {
        let mut threads_pool = Vec::new();
        let chunks = split_to(items, threshold);

        for chunk in chunks {
            threads_pool.push(std::thread::spawn(move || -> Vec<R> {
                let mut chunk_interm_result = Vec::new();
                for item in chunk {
                    chunk_interm_result.push(proc(item));
                }
                chunk_interm_result
            }));
        }

        for thread in threads_pool {
            let thread_result = thread.join().unwrap();
            result.extend(thread_result);
        }
    } else {
        for item in items {
            result.push(proc(item));
        }
    }
    result
}

fn split_to<T>(mut items: Vec<T>, threshold: usize) -> Vec<Vec<T>> {

    let mut result = Vec::new();

    if items.len() <= threshold {
        result.push(items)
    } else {
        while !items.is_empty() {
            let chunk: Vec<T> = items.drain(..threshold.min(items.len())).collect();
            result.push(chunk);
        }
    }
    result
}  

#[cfg(test)]
#[test]
fn test_parallel_comp() {
    let test_vec1 = parallel_computing(vec![1, 2, 3, 4], |x| x * x); //1 work thread
    let test_vec2 = parallel_computing(
        vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8,
            9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
        ],
        |x| x * x,
    ); //3 work threads

    assert_eq!(test_vec1, vec![1, 4, 9, 16]);
    assert_eq!(
        test_vec2,
        vec![
            1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 1, 4, 9,
            16, 25, 36, 49, 64, 81, 100, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100
        ]
    );
}
