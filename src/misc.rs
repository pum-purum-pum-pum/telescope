use rand::Rng;
use std::{thread, time};

pub fn dummy_sleep(nanos: u64) {
    let ten_millis = time::Duration::from_nanos(nanos);
    thread::sleep(ten_millis);
}

// TODO remove depth somehow (we don't want to call the function with 0 depth)
pub fn generate_spans(depth: usize, max_depth: usize) {
    let mut rng = rand::thread_rng();
    dummy_sleep(1);
    if depth == max_depth {
        // let p = 0.5;
        // dummy_sleep(10);
        return;
    }
    if depth > max_depth / 2 {
        let p = 0.5;
        if rng.gen_range(0.0, 1.0) > 1. - p {
            return;
        }
    }
    let max_spans = if depth < 4 {4} else {3};
    for i in 0..rng.gen_range(1, max_spans) {
        let name = format!("span_{}_{}", depth, i);
        flame::start(name.clone());
        generate_spans(depth + 1, max_depth);
        flame::end(name.clone());
    }
}

pub fn _test_spans() {
    flame::start("all");
    dummy_sleep(10);
    {
        flame::start("inside1");
        dummy_sleep(20);
        flame::end("inside1");
        flame::start("inside2");
        dummy_sleep(40);
        {
            flame::start("deep_inside1");
            dummy_sleep(20);
            flame::end("deep_inside1");
            flame::start("deep_inside2");
            dummy_sleep(50);
            flame::end("deep_inside2");
        }
        flame::end("inside2");
    }
    flame::end("all");
}
