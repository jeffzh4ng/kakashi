use std::{cmp, io};

pub fn solve_292_a() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();

        let tc = line
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((tc[0], tc[1]))
    }

    let mut last_time = 0;
    let mut queue = 0;
    let mut max_queue_size = 0;
    for (t, c) in input {
        if queue > 0 {
            if t - last_time > queue {
                queue = 0;
            } else {
                queue -= t - last_time;
            }
        }
        queue += c;
        max_queue_size = cmp::max(max_queue_size, queue);

        last_time = t;
    }
    last_time = last_time + queue;

    println!("{:?} {:?}", last_time, max_queue_size);
}

pub fn solve_174_a() {}
