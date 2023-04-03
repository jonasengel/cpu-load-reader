use std::{time, thread};
use std::fs::File;
use std::io::Read;

fn main() {
    let half_sec = time::Duration::from_millis(5000);
    let mut old_reading = Vec::new();
    let mut new_reading = Vec::new();
    let mut diff_reading = vec![0.0; 10];
    old_reading = cast_to_f64();
    loop {
        thread::sleep(half_sec);
        let old_sum = old_reading.iter().sum::<f64>().to_owned();
        let res = cast_to_f64();
        let sum = &res.iter().sum::<f64>().to_owned();
        let delta_sum = sum - old_sum;
        new_reading = cast_to_f64();
        for i in 0..10 {
            diff_reading[i] = (new_reading[i] - old_reading[i])/delta_sum;
            println!("{}, {}", diff_reading[i], delta_sum);
        }
        old_reading = new_reading;
        println!("this was one complete reading");
    }
}

fn read_proc_stat() -> String {
    let mut ps_file = File::open("/proc/stat")
    .expect("some failure when opening proc/stat");
    let mut ps_reading = String::new();
    ps_file.read_to_string(&mut ps_reading)
    .expect("proc stat could not be read");
    let ps_first_lines: Vec<&str> = ps_reading.lines().collect();
    let result = ps_first_lines[0].to_owned();
    return result
}

fn cast_to_f64() -> Vec<f64> {
    let readings = read_proc_stat();
    let mut vec_readings: Vec<&str> = readings.split_whitespace().collect();
    vec_readings.remove(0);
    let result: Vec<f64> = vec_readings.iter().flat_map(|x| x.parse::<f64>()).collect();
    return result
}
