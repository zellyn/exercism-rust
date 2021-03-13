use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Multiple publishers, single consumer means we can have one channel for the results…
    let (result_tx, result_rx) = mpsc::channel();
    // …but need one input channel per worker.
    let mut channels: Vec<mpsc::Sender<&str>> = Vec::with_capacity(worker_count);

    // Spin up workers
    for _ in 0..worker_count {
        let result_tx = result_tx.clone();

        let (input_tx, input_rx) = mpsc::channel();
        channels.push(input_tx);
        thread::spawn(move || {
            let mut counts: HashMap<char, usize> = HashMap::new();

            for s in input_rx {
                for ch in s.chars() {
                    let counter = counts.entry(ch).or_insert(0);
                    *counter += 1;
                }
            }
            // When done
            result_tx.send(counts).unwrap();
        });
    };

    // Send in the inputs
    // for chunk in input.into_iter().chunks(worker_count) {
    //     chunk.iter().enumerate().for_each(|(i, &s)| {
    //         channels[i].send(s).unwrap()
    //     })
    // }
    for (i, &s) in input.into_iter().enumerate() {
        channels[i % worker_count].send(s).unwrap();
    }

    // Close all input channels
    drop(channels);

    result_rx.iter().take(worker_count).fold(HashMap::new(), |mut ha, hb| {
        for (ch, count) in hb {
            let counter = ha.entry(ch).or_insert(0);
            *counter += count;
        }
        ha
    })
}
