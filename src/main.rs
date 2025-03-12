use std::collections::HashSet;

fn optimal_page_replacement(pages: Vec<i32>, frame_size: usize) -> usize {
    let mut frames = HashSet::new();
    let mut page_faults = 0;

    for (i, page) in pages.iter().enumerate() {
        if !frames.contains(page) {
            // Page fault
            if frames.len() == frame_size {
                // Find the page to replace
                let mut furthest = None;
                let mut max_distance = 0;

                for &frame in &frames {
                    let distance = pages[i + 1..].iter().position(|&x| x == frame);
                    let future_distance = distance.unwrap_or(pages.len());
                    if future_distance > max_distance {
                        furthest = Some(frame);
                        max_distance = future_distance;
                    }
                }

                if let Some(to_remove) = furthest {
                    frames.remove(&to_remove);
                }
            }

            frames.insert(*page);
            page_faults += 1;
        }
    }

    page_faults
}

fn main() {
    let pages = vec![7, 0, 1, 2, 0, 3, 0, 4, 2, 3, 0, 3, 2];
    let frame_size = 3;

    let faults = optimal_page_replacement(pages.clone(), frame_size);
    println!("Number of page faults: {}", faults);
}
