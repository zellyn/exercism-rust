use crossbeam_utils::thread;
use std::cmp;
use std::collections::HashMap;
use std::sync::mpsc;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (size, remainder) = (input.len() / worker_count, input.len() % worker_count);
    let actual_worker_count = cmp::min(worker_count, input.len());

    // Multiple publishers, single consumer means we can have one channel for the results.
    let (result_tx, result_rx) = mpsc::channel();

    thread::scope(|scope| {
        let mut pos = 0;
        for i in 0..actual_worker_count {
            let chunk_size = if i < remainder { size + 1 } else { size };
            let chunk = &input[pos..pos + chunk_size];
            pos += chunk_size;
            let result_tx = result_tx.clone();
            scope.spawn(move |_| {
                let mut counts: HashMap<char, usize> = HashMap::new();
                for s in chunk {
                    for ch in s
                        .chars()
                        .filter(|c| c.is_alphabetic())
                        .map(|c| c.to_lowercase())
                        .flatten()
                    {
                        *counts.entry(ch).or_insert(0) += 1;
                    }
                }
                result_tx.send(counts).unwrap();
            });
        }
    })
    .unwrap();

    // Merge the results
    result_rx
        .iter()
        .take(actual_worker_count)
        .fold(HashMap::new(), |mut ha, hb| {
            for (ch, count) in hb {
                *ha.entry(ch).or_insert(0) += count;
            }
            ha
        })
}
