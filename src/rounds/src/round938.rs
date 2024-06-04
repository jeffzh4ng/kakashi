use std::{collections::HashMap, io};

// 7 4 2
// a:4 1 2 3 4 5 6

//   4 1 2 3
//     1 2 3 4
//       2 3 4 5
//         3 4 5 6

// b:1 2 3 4
// o:4

// 7 4 3

// a:4 1 2 3 4 5 6

//   4 1 2 3
//     1 2 3 4
//       2 3 4 5

// b:1 2 3 4
// o:3

// 7 4 4
// a:4 1 2 3 4 5 6
// b:1 2 3 4
// o:2

// idea:

//      set()
//        |
// |sliding window|

// 4 1 1
// 4 1 5 6
// 6

// 11, 5, 3
// 9 9 2 2 10 9 7 6 3 6 3

// 9 9 2 2 10 n
//   9 2 2 10 9 n
//     2 2 10 9 7 y
//       2 10 9 7 6 y
//         10 9 7 6 3 y
//            9 7 6 3 6

// 6 9 7 8 10

pub fn solve_d() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        let mut line_two = String::new();
        let mut line_three = String::new();

        io::stdin().read_line(&mut line_one).unwrap();
        io::stdin().read_line(&mut line_two).unwrap();
        io::stdin().read_line(&mut line_three).unwrap();

        let nmk = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (n, m, k) = (nmk[0], nmk[1], nmk[2]);

        let input_a = line_two
            .trim()
            .split(" ")
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let input_b = line_three
            .trim()
            .split(" ")
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((n, m, k, input_a, input_b))
    }

    let mut output = Vec::new();
    for tc in input {
        let (_, m, k, input_a, input_b) = tc;
        let map_b = input_b.iter().fold(HashMap::new(), |mut map, &b| {
            *map.entry(b).or_insert(0) += 1;
            map
        });

        let mut map_a = input_a
            .iter()
            .take(m as usize)
            .fold(HashMap::new(), |mut map, &a| {
                *map.entry(a).or_insert(0) += 1;
                map
            });

        let mut good_subsegments = 0;
        let (mut l, mut r) = (0, (m - 1) as usize);
        while r < input_a.len() {
            let match_count = map_a.iter().fold(0, |count, (k, &v)| {
                if map_b.contains_key(k) && *map_b.get(k).unwrap() <= v {
                    count + 1
                } else {
                    count
                }
            });
            // println!("{:?}{:?}{:?}", map_a, map_b, match_count);

            if match_count >= k {
                good_subsegments += 1;
            }

            // dec input_a[l]
            // todo: or_default even though i know it's in there?
            *map_a.entry(input_a[l]).or_default() -= 1;
            if *map_a.get(&input_a[l]).unwrap() == 0 {
                map_a.remove(&input_a[l]);
            }
            l += 1;

            // inc input_a[r]
            r += 1;
            if r < input_a.len() {
                *map_a.entry(input_a[r]).or_insert(0) += 1;
            }
        }

        output.push(good_subsegments);
    }

    for o in output {
        println!("{:?}", o);
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

        let nk = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (n, k) = (nk[0], nk[1]);

        let input_a = line_two
            .trim()
            .split(" ")
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((n, k, input_a))
    }

    let mut output = Vec::new();
    for tc in input {
        let (_, mut k, mut input_a) = tc;

        // n: ship order st. n <= 1e5
        // a: durability order st. a_i <= 1e9
        // k: attack order st. ki <= 1e15

        // n:2 k:2
        // a: 3 2

        // idea1: two pointers, O(n) TLE
        let mut sunk = 0;
        let (mut l, mut r) = (0 as usize, input_a.len() - 1);
        while k > 0 {
            input_a[l] -= 1;
            if input_a[l] == 0 {
                l += 1;
                sunk += 1;
            }
            k -= 1;

            if k == 0 {
                break;
            }

            input_a[r] -= 1;
            if input_a[r] == 0 {
                r -= 1;
                sunk += 1;
            }
            k -= 1;
        }

        // idea 2:
        // -log(n) doesn't seem possible, input_a is not sorted.

        // -how do i step k down more than 1 unit at a time?
        //   -encoding rules to sink both at once doesn't seem possible
        //   -once the left sinks, the alternation makes it so the kraken starts attacking left+1
        //   -it's not a stepwise attack on both l and r until both are sunk.

        // -can we somehow compress? preprocessing?

        // n:4 k:6
        // 1 2 4 3
        //   l r

        output.push(sunk);
    }

    for o in output {
        println!("{:?}", o);
    }
}

pub fn solve_b() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        let mut line_two = String::new();

        io::stdin().read_line(&mut line_one).unwrap();
        io::stdin().read_line(&mut line_two).unwrap();

        let ncd = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (n, c, d) = (ncd[0], ncd[1], ncd[2]);

        let input_b = line_two
            .trim()
            .split(" ")
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((n, c, d, input_b))
    }

    //n:3,c:2,d:3

    // 3 9 6
    // 5 7 1
    // 11 4 8

    // n:2 c:100 d:100
    // 400 300
    // 400 500

    let mut output = Vec::new();
    for tc in input {
        let (n, c, d, input_b) = tc;
        let n = n as usize;

        // -row prop (1)
        // -col prop (2)
        // a11 has to be the min, since (1) and (2) ==> elements strictly increasing

        // set_b = set(input_b)
        // find the min.
        // reconstruct the progressive square a
        // set_a
        // return set_a == set_b

        // blunder: think about maps and sets, when making a choice of one over
        // the other, try to find counterexamples with the other.

        // 4 4 4

        //
        // 23
        //
        //

        // 3  7  11 15
        // 7  11 15 19
        // 11 15 19 23
        // 15    23 27

        // let set_b = input_b.iter().collect::<HashSet<_>>();
        let map_b = input_b.iter().fold(HashMap::new(), |mut map, &x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });
        let min = *(map_b.keys().min().unwrap());
        let mut input_a = vec![0; input_b.len()];
        input_a[0] = min;

        // It can be shown that for any values of n, min, c, and d, there exists
        // exactly one progressive square that satisfies all the rules

        for i in 0..input_a.len() {
            // set right
            if (i + 1) % n != 0 {
                input_a[i + 1] = input_a[i] + d;
            }

            // set down
            if (i + n) < input_b.len() {
                input_a[i + n] = input_a[i] + c;
            }

            // todo, we only have to set first row
            // then everything else is determined by column
        }

        // let set_a = input_a.iter().collect::<HashMap<_>>();
        let map_a = input_a.iter().fold(HashMap::new(), |mut map, &x| {
            *map.entry(x).or_insert(0) += 1;
            map
        });

        if map_a == map_b {
            output.push("YES")
        } else {
            output.push("NO")
        }

        // if set_a.is_subset(&set_b) && set_b.is_subset(&set_a) {
        //     output.push("YES")
        // } else {
        //     output.push("NO")
        // }
    }

    for o in output {
        println!("{o}")
    }
}

pub fn solve_a() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();

        io::stdin().read_line(&mut line_one).unwrap();

        let nab = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (n, a, b) = (nab[0], nab[1], nab[2]);

        input.push((n, a, b))
    }

    let mut output = Vec::new();
    for tc in input {
        let (n, a, b) = tc;
        let amount_with_discount = (n / 2 * b) + (n % 2 * a);
        let amount_without_discount = n * a;
        let min_amount = std::cmp::min(amount_with_discount, amount_without_discount);
        output.push(min_amount);
    }

    for o in output {
        println!("{o}")
    }
}
