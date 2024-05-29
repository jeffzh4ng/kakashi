use std::{collections::HashMap, io};

pub fn solve_178_a1() -> Result<(), io::Error> {
    let mut line_one = String::new();
    let mut line_two = String::new();

    io::stdin().read_line(&mut line_one).unwrap();
    io::stdin().read_line(&mut line_two).unwrap();

    let n = line_one.trim().parse::<usize>().unwrap();
    let input = line_two
        .trim()
        .split(" ")
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // choose (i,t)
    //   st:
    //     - a_i > 0
    //     - i+2^t <= n
    //   so then
    //     - a_i         -= 1
    //     - a_{i+2^t}   += 1

    // output = [b_1,...,b_n]
    //  st: b_i = #min moves ==> a_1,...a_i all zero.

    // tc1
    // -----------
    //                              178_a1()
    // n=4                   ===============> 1
    // as=[1,0,1,2]                           1
    //                                        3 why is this value 3? shouldn't it be 2?

    // it's 3 because when k=0, you're forced to increment a_2 by 1 with (i:0,i:2)

    // 1012

    // k=0
    //   - i:0,t:1 or i:0,t:2
    // k=1
    //   - same as above. just need to set a_0 to 0
    // k=2
    //   - i:0,t:1=>2^t=2 (to set a_0 to 0) --> [1,0,2,2]
    //   - i:2,t:0 (to set a_2 to 1) --> [1,0,1,3]
    //   - i:2,t:0 (to set a_2 to 0) --> [1,0,0,4]

    // tc2
    // -----------
    //                              178_a1()
    // n=8                    ===============> 1
    // as=[1,2,3,4,5,6,7,8]                    3
    //                                         6
    //                                         10
    //                                         16 why is this value 16? shouldn't it be 15?
    //                                         24
    //                                         40

    // idea 1: naive heuristic
    //         minimize ai to 0
    //         2^t > 0 ==> i+2^t > i, ==> indices 1...ai-1 will never be updated (no relapses)

    // for i in 1..k {
    //  i is chosen for us. we need a_i to be 0.

    //   **idea 1**:
    //   finding good values of t: <-----------------------crux of the problem
    //     - foo = set(find all the elements that are 0 already)
    //     - filter out all values of t st foo.contains(i+2^t)

    //    **observations 1**:
    //    **instead, can we just set t st i+2^t > k?**
    //    b/cwe never want t <= k b/c all a_i are non-negative (we never want to increment an a_i st i<k)
    //
    //    yes, that's the goal, but sometimes we are forced to increment elements
    //    that we need to later decrement b/c step size of t is 2.
    //    that is, when minimizing earlier elements, we are forced to increment
    //    in step sizes of 2, which is problematic for n-1 when n is odd.

    //         you want to choose t=1 if you're minimizing the last element in the sequence of length k
    //         you want to choose t>=2 if the next element is 0 (you don't want to touch it)

    // }

    // idea 2:
    // if (i%2 == n%2) OR (i = n-1):
    //   then the last garbage element is accessible via steps of 2^t
    // else:
    //   what t do we choose to minimize damage?
    //   we should choose a t st
    //     - a[i+2^t] != 0
    //     - (i+2^t % 2) == (n%2) [has same parity as n]
    //       so that a_{i+2^t} can use the garbage element

    fn choice_of_t(input: &[i32], i: usize) -> u32 {
        let n = input.len() - 1;

        if (n - i) % 2 == 0 {
            //   then the last garbage element is accessible via steps of 2^t

            // n = i+2^t
            // n-i= 2^t
            // log(n-i) = tlog2
            // t = log(n-i)/log(2)
            // println!("mooo {:?}/{:?}", ((n - i) as f32).log2(), (2 as f32).log2());
            (((n - i) as f32).log2() / (2 as f32).log2()) as u32
        } else {
            //   we should choose a t st
            //     - a[i+2^t] != 0
            //     - (i+2^t % 2) == (n%2) [has same parity as n]
            //       so that a_{i+2^t} can use the garbage element
            let mut possible_ts = Vec::new();
            for t in 0..=(((n - i) as f32).log2() as usize) {
                let i_plus_step = i + i32::pow(2, t as u32) as usize;
                if i_plus_step <= n && input[i_plus_step] == 0
                // || ((i_plus_step % 2 != n % 2) && (i_plus_step != n - 1))
                {
                    // println!(
                    //     "passing t value: {t} ==> i+2^t={:?}",
                    //     i + i32::pow(2, t as u32) as usize
                    // );
                    continue;
                } else if i_plus_step <= n {
                    // println!("i_plus_step {i_plus_step} is less than {n}");
                    possible_ts.push(t);
                }
            }

            // how do we choose between multiple ts?
            // it doesn't seem to matter since step size with minimization is constant (-1)
            // println!("choice of ts: {:?}", possible_ts);

            if let Some(t) = possible_ts.last() {
                *t as u32
            } else {
                panic!()
            }
        }
    }

    let mut output = Vec::new();
    for k in 0..n - 1 {
        let mut working_input = input.clone();
        let mut min_moves = 0;

        // println!("for k={k}");
        for i in 0..=k {
            // println!("working_input, {:?}, i: {i}", working_input);
            while working_input[i] > 0 {
                let t = choice_of_t(&input, i as usize);
                // let foo = i + i32::pow(2, t as u32) as usize;
                // println!(
                //     "going to access index {foo} bc {i}+{:?}",
                //     i32::pow(2, t as u32) as usize
                // );

                working_input[i] -= 1;
                working_input[i + i32::pow(2, t as u32) as usize] += 1;
                min_moves += 1;
                // println!(
                //     "choice (i:{i},t:{t}), updated working_input to {:?}",
                //     working_input
                // );
            }
        }
        output.push(min_moves);
    }

    for x in output {
        println!("{:?}", x);
    }
    Ok(())
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct CountToIndexTuple((i32, usize));

impl Ord for CountToIndexTuple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub fn solve_1970_a1() -> Result<(), io::Error> {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();

    let input = line_one.trim().chars().collect::<Vec<_>>();
    let mut open_counts = Vec::with_capacity(input.len());
    let mut open_count = 0;

    for (i, c) in input.iter().enumerate() {
        match c {
            '(' => {
                open_counts.push((open_count, i));
                open_count += 1;
            }
            ')' => {
                open_counts.push((open_count, i));
                open_count -= 1;
            }
            _ => panic!(),
        }
    }
    open_counts.sort();

    let output = open_counts
        .iter()
        .map(|(_, i)| input[*i])
        .collect::<String>();
    println!("{output}");

    Ok(())
}
