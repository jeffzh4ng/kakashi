use std::io;

pub fn solve_a() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();

        io::stdin().read_line(&mut line_one).unwrap();

        let abc = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (a, b, c) = (abc[0], abc[1], abc[2]);

        input.push((a, b, c))
    }

    let mut output = Vec::new();
    for tc in input {
        let (i, mut e, mut u) = tc;

        e = if e % 3 != 0 {
            // top up with universals

            let tmp = 3 - (e % 3);
            e += tmp;
            u -= tmp;

            if u < 0 {
                -1
            } else {
                e
            }
        } else {
            e
        };
        // println!("{:?}/({i},{e},{u})", tc);

        let amount = if e == -1 {
            -1
        } else {
            i + (e / 3)
                + if u > 0 {
                    std::cmp::max(u32::div_ceil(u as u32, 3) as i32, 1)
                } else {
                    0
                }
        };

        output.push(amount);
    }

    for o in output {
        println!("{o}")
    }
}

pub fn solve_b() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        io::stdin().read_line(&mut line_one).unwrap();

        let abm = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let (a, b, m) = (abm[0], abm[1], abm[2]);

        input.push((a, b, m))
    }

    // blunder: get to examples quicker
    // 6 7 4 -> 2
    // 3 4 10 -> 7
    // 7 8 56 -> 17
    // 5 6 78123459896 -> 28645268630
    // 1 1 1 -> 4
    // 1 1 1000000000000000000 -> 2000000000000000002

    // obs:
    // - calculate overlap with itself A
    //     overlap = 0
    //     while r_a<a+m:
    //       overlap += 1
    //
    // - calculate overlap with itself B
    //     overlap = 0
    //     while r_b<b+m:
    //       overlap += 1
    //

    // - calculate overlap with other
    //     overlap = 0
    //     while r_a<b+m

    // 3 4 10 -> 7
    // (3+10)/3
    // (4+10)/4
    //           A     A       A
    //     A------------------------
    //       B-------------------------
    // 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21

    let mut output = Vec::new();
    for tc in input {
        let (a, b, m) = tc;

        // 6 7 4 -> 2

        // 3 4 10 -> 7

        //     -------------------------
        //     A     A     A       A

        //       --------------------------
        //       B       B         B           B
        // 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21

        // 1 1 1 -> 4       (2)

        // ---
        // A A
        // ---
        // B B
        // 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21

        // no need to account for your own tail
        let mut overlap = 0;
        let overlap_a = (a + m) / a;
        let overlap_b = (b + m) / b;
        // println!("{:?},{:?}", overlap_a, overlap_b);
        overlap += overlap_a + overlap_b;

        // you don't have to account for the other's tail when you're calculating both
        // let overlap_ab = u64::div_ceil(a + m, b);
        // let overlap_ba = u64::div_ceil(b + m, a);
        // let mx = std::cmp::max(overlap_ab, overlap_ba);
        // overlap += mx;

        // println!(
        //     "{:?},{:?},{:?}:{:?},{:?},{:?},{:?}",
        //     a, b, m, overlap_a, overlap_b, overlap_ab, overlap_ba
        // );

        output.push(overlap);
    }

    for o in output {
        println!("{o}")
    }
}

pub fn solve_c() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        let mut line_two = String::new();
        io::stdin().read_line(&mut line_one).unwrap();
        io::stdin().read_line(&mut line_two).unwrap();

        let input_a = line_two
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        input.push(input_a)
    }

    let mut output = Vec::new();
    for tc in input {
        // collect the sum
        let homes = tc;
        let mut right_ones: u64 = homes.iter().sum();
        let mut left_zeros = 0;

        let mut possible_is = Vec::new();
        if (homes.len() / 2) as u64 <= right_ones {
            possible_is.push(0);
        }

        if homes[0] == 0 {
            left_zeros += 1;
        } else {
            right_ones -= 1;
        }

        for i in 1..homes.len() {
            let n_l = i as u64;
            let n_r = (homes.len() - i) as u64;
            if u64::div_ceil(n_l, 2) <= left_zeros && u64::div_ceil(n_r, 2) <= right_ones {
                possible_is.push(i);
            }

            // visit element, for next shift over
            if homes[i] == 0 {
                left_zeros += 1;
            } else {
                right_ones -= 1;
            }
        }

        if (homes.len() / 2) as u64 <= left_zeros {
            possible_is.push(homes.len());
        }

        println!("{:?}", possible_is);

        // find the i closest to the middle
        let mut closest_distance = 0 as f64;
        let mut closest_i = 0;
        for i in possible_is {
            let distance = f64::abs((homes.len() / 2) as f64 - i as f64);
            println!("{:?}:{:?}", i, distance);
            if distance < closest_distance {
                closest_distance = distance;
                closest_i = i;
            }
        }

        output.push(closest_i);
    }
    for o in output {
        println!("{o}");
    }
}
