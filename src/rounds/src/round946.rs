use std::io;

pub fn solve_a() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        io::stdin().read_line(&mut line_one).unwrap();
        let xy = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((xy[0], xy[1]))
    }

    let mut output = Vec::new();
    for tc in input {
        let mut num_screens = 0;
        let (mut num_onebyones, mut num_twobytwos) = tc;

        while num_onebyones > 0 || num_twobytwos > 0 {
            let mut space = 15;
            if num_twobytwos >= 2 {
                space -= 4 * 2;
                num_twobytwos -= 2;
            } else if num_twobytwos == 1 {
                space -= 4;
                num_twobytwos -= 1;
            }

            num_onebyones -= space;
            num_screens += 1;
        }

        output.push(num_screens);
    }

    for o in output {
        println!("{o}");
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
        let n = line_one.trim().parse::<usize>().unwrap();

        let mut line_two = String::with_capacity(n);
        io::stdin().read_line(&mut line_two).unwrap();

        input.push(line_two.trim().chars().collect::<String>());
    }

    let mut output = Vec::new();
    for i in input {
        let mut sorted = i.chars().collect::<Vec<_>>();
        sorted.sort();
        sorted.dedup();
        let mut sorted_map = std::collections::HashMap::new();
        // println!("{:?}", sorted);
        // alt: or two pointers (prob safer to implement without index parity bugs)
        for i in 0..=(sorted.len() / 2) {
            // blunder:
            // it's not necessary that the rhs of sorted is the ouput of enc->dec
            // the rhs can be input, so the map needs to be two sided.
            sorted_map.insert(sorted[i], sorted[sorted.len() - 1 - i]);
            sorted_map.insert(sorted[sorted.len() - 1 - i], sorted[i]);
        }

        let mut decoded = Vec::with_capacity(i.len());
        for c in i.chars() {
            // println!("{c}, {:?}", sorted_map);
            decoded.push(sorted_map.get(&c).unwrap());
        }
        output.push(decoded.into_iter().collect::<String>());
    }

    for o in output {
        println!("{o}");
    }
}

pub fn solve_c() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut inputs = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        io::stdin().read_line(&mut line_one).unwrap();
        let n = line_one.trim().parse::<usize>().unwrap();

        let mut line_two = String::with_capacity(n);
        io::stdin().read_line(&mut line_two).unwrap();

        let line_two_parsed = line_two
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        inputs.push(line_two_parsed);
    }

    // meta blunder: coding, not thinking
    let mut outputs = Vec::new();
    for input in inputs {
        let mut triples = std::collections::HashMap::new();
        // collect triples
        for i in 1..input.len() - 1 {
            let triple = (input[i - 1], input[i], input[i + 1]);
            let prev: usize = *triples.entry(triple).or_default();
            triples.insert(triple, prev + 1);
        }

        // count beautiful triples
        let mut output = 0;
        let mut seen = std::collections::HashSet::new();
        for (t, n_t) in &triples {
            // idea 1: brute force (pairwise comparison~n^2)
            // idea 2: sort? how would i sort?
            for (u, n_u) in &triples {
                if seen.contains(&(u, t)) {
                    continue;
                }
                let p1 = t.0 != u.0 && t.1 == u.1 && t.2 == u.2;
                let p2 = t.0 == u.0 && t.1 == u.1 && t.2 != u.2;
                let p3 = t.0 == u.0 && t.1 != u.1 && t.2 == u.2;

                if p1 || p2 || p3 {
                    output += n_t * n_u; // blunder: +, max, then *
                    seen.insert((t, u));
                }
            }
        }

        outputs.push(output);
    }

    for o in outputs {
        println!("{o}");
    }
}
