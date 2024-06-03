use std::io;

pub fn solve_a() -> Result<(), io::Error> {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        return gcd(b, a % b);
    }

    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        input.push(line.trim().parse::<i32>().unwrap());
    }

    let mut output = Vec::new();
    for x in input {
        let mut max_gcd = 0;
        let mut max_y = 0;

        for y in 1..x {
            let tmp = gcd(x, y) + y;

            if tmp > max_gcd {
                max_gcd = tmp;
                max_y = y;
            }

            // println!("{y}{max_gcd}");
        }
        output.push(max_y);
    }

    for o in output {
        println!("{o}");
    }

    return Ok(());
}

// idea 1: brute force -> n^2 -> TLE
// for every a_k <-- maximize prefix length (1)
//   walk i and j down a and b <-- check if subsequence (2)

// how do i get this down to n?
// - both (1) and (2) seem necessary
// - (1): i need to check every a_k in order to maximize f(a,b)
// - (2):   i need to check if a_k is a subsequence of b

// is there some way i can combine these two steps?

// - obs: i can combine these two steps by breaking down the unit from &[str] -> char
// - since the property to satisfy is subsequence, i will get the max prefix
//   by maximizing pointers i and j in a forwards fashion
// - there doesn't need to be any restart

pub fn solve_b() -> Result<(), io::Error> {
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

        // let nm = line_one
        //     .trim()
        //     .split(" ")
        //     .map(|c| c.parse::<usize>().unwrap())
        //     .collect::<Vec<_>>();

        input.push((
            line_two
                .trim()
                .parse::<String>()
                .unwrap()
                .as_bytes()
                .to_owned(),
            line_three
                .trim()
                .parse::<String>()
                .unwrap()
                .as_bytes()
                .to_owned(),
        ))
    }

    let mut output = Vec::new();
    for tc in input {
        let (a, b) = tc;

        // blunder: indexes with substrings
        // println!("moose: {a},{a_prefix}");

        let mut i = 0;
        let mut j = 0;

        while i < a.len() && j < b.len() {
            if a[i] == b[j] {
                i += 1;
            }

            j += 1;
        }

        // println!("moose:{:?}{i}", a);
        output.push(i);
    }

    for o in output {
        println!("{o}");
    }

    return Ok(());
}

pub fn solve_c() -> Result<(), io::Error> {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        let mut line_two = String::new();

        io::stdin().read_line(&mut line_one).unwrap();
        io::stdin().read_line(&mut line_two).unwrap();

        // let nm = line_one
        //     .trim()
        //     .split(" ")
        //     .map(|c| c.parse::<usize>().unwrap())
        //     .collect::<Vec<_>>();

        let input_x = line_two
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push(input_x);
    }

    let mut output = Vec::new();
    for input_x in input {
        let mut o = Vec::with_capacity(input_x.len() + 1);
        o.push(0);

        // idea 1
        // select a_is from the back

        // todo: how does reversing range work?
        // will it start at n-1 and reach 0?
        for _ in (0..o.len()).rev() {
            // in: 1 < x_i <= 500
            // out: 1 <= a_i <= 1e9

            // find a_i and a_{i-1} st
            //  -1 <= a_i <= 1e9
            //  -mod(a_i, a_{i-1}) = x[i]

            //  ---idea 1:
            //    - x_i

            // - we can't brute force through a space of 1e9
            // - results with primes?
        }

        output.push(o);
    }

    for o in output {
        println!("{:?}", o);
    }

    Ok(())
}

pub fn solve_d() -> Result<(), io::Error> {
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

        let nkbs = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let k = nkbs[1];
        let b = nkbs[2];
        let s = nkbs[3];

        let input_p = line_two
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>(); // todo: allocate with n,k..and then parse

        let input_a = line_three
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((k, b, s, input_p, input_a));
    }

    for _ in input {
        // let (k, b, s, input_p, input_a) = tc;

        // 1 <= k <= 1e9
        // 1 <= p_i <= 2e5
        // 1 <= a_i <= 1e9

        // - can i even iterate k to 1e9? TLE?
        // - how does a_i > 2e5 even move from x to p_x

        // trying to win ==> maximizing a_i?
    }

    Ok(())
}
