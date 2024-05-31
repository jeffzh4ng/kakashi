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
        let mut triples: std::collections::HashMap<(i32, i32, i32), usize> =
            std::collections::HashMap::new();
        let mut dim1: std::collections::HashMap<i32, ((i32, i32), usize)> =
            std::collections::HashMap::new();
        let mut dim2: std::collections::HashMap<i32, ((i32, i32), usize)> =
            std::collections::HashMap::new();
        let mut dim3: std::collections::HashMap<i32, ((i32, i32), usize)> =
            std::collections::HashMap::new();

        // collect triples
        for i in 1..input.len() - 1 {
            let triple = (input[i - 1], input[i], input[i + 1]);
            let xyz = (triple.0, (triple.1, triple.2));
            let yxz = (triple.1, (triple.0, triple.2));
            let zxy = (triple.2, (triple.0, triple.1));

            // mega blunder:
            // -when building x->yz, y->zx, and z->xy maps,
            // -using the same key.
            // for  x->yz, (2,2,3) and (2,2,2) will have the same entry
            // - we need separate entries

            // x->y->z
            // 2->2->3
            // 2->2->2

            // micro blunder: not knowing get or default
            let prev = if triples.contains_key(&triple) {
                *triples.get(&triple).unwrap()
            } else {
                0
            };
            let prev1 = if dim1.contains_key(&xyz.0) {
                if (*dim1.get(&xyz.0).unwrap()).0 == xyz.1 {
                    (*dim1.get(&xyz.0).unwrap()).1
                } else {
                    0
                }
            } else {
                0
            };
            let prev2 = if dim2.contains_key(&yxz.0) {
                if (*dim2.get(&yxz.0).unwrap()).0 == yxz.1 {
                    (*dim2.get(&yxz.0).unwrap()).1
                } else {
                    0
                }
            } else {
                0
            };
            let prev3 = if dim3.contains_key(&zxy.0) {
                if (*dim3.get(&zxy.0).unwrap()).0 == zxy.1 {
                    (*dim3.get(&zxy.0).unwrap()).1
                } else {
                    0
                }
            } else {
                0
            };
            triples.insert(triple, prev + 1);
            dim1.insert(xyz.0, (xyz.1, prev1 + 1));
            dim2.insert(yxz.0, (yxz.1, prev2 + 1));
            dim3.insert(zxy.0, (zxy.1, prev3 + 1));
        }
        println!("{:?}", dim1);
        println!("{:?}", dim2);
        println!("{:?}", dim3);

        // count beautiful triples

        // idea 1: brute force, pairwise comparison~n^2
        let mut output = 0;
        // let mut seen = std::collections::HashSet::new();
        for (t, n_t) in &triples {
            // idea 2: closest neighbors

            // x
            if dim1.contains_key(&(t.0 + 1)) {
                let (u_yz, n_u) = dim1.get(&(t.0 + 1)).unwrap();
                let u = ((t.0 + 1), u_yz.0, u_yz.1);
                if t.1 == u.1 && t.2 == u.2 {
                    output += n_t * n_u;
                }
            }
            if dim1.contains_key(&(t.0 - 1)) {
                let (u_yz, n_u) = dim1.get(&(t.0 - 1)).unwrap();
                let u = ((t.0 - 1), u_yz.0, u_yz.1);
                if t.1 == u.1 && t.2 == u.2 {
                    output += n_t * n_u;
                }
            }

            // y
            if dim2.contains_key(&(t.1 + 1)) {
                let (u_yz, n_u) = dim2.get(&(t.1 + 1)).unwrap();
                let u = (u_yz.0, (t.0 + 1), u_yz.1);
                if t.0 == u.0 && t.2 == u.2 {
                    output += n_t * n_u;
                }
            }
            if dim2.contains_key(&(t.1 - 1)) {
                let (u_yz, n_u) = dim2.get(&(t.1 - 1)).unwrap();
                let u = (u_yz.0, (t.0 - 1), u_yz.1);
                if t.0 == u.0 && t.2 == u.2 {
                    output += n_t * n_u;
                }
            }

            // z
            if dim3.contains_key(&(t.2 + 1)) {
                let (u_yz, n_u) = dim3.get(&(t.2 + 1)).unwrap();
                let u = (u_yz.0, u_yz.1, (t.2 + 1));
                if t.0 == u.0 && t.1 == u.1 {
                    output += n_t * n_u;
                }
            }
            if dim3.contains_key(&(t.2 - 1)) {
                let (u_yz, n_u) = dim3.get(&(t.2 - 1)).unwrap();
                let u = (u_yz.0, u_yz.1, (t.2 - 1));
                if t.0 == u.0 && t.1 == u.1 {
                    output += n_t * n_u;
                }
            }

            // for (u, n_u) in &triples {
            //     // blunder: seen has to be set<pair>
            //     if seen.contains(&(u, t)) {
            //         continue;
            //     }
            //     let p1 = t.0 != u.0 && t.1 == u.1 && t.2 == u.2;
            //     let p2 = t.0 == u.0 && t.1 == u.1 && t.2 != u.2;
            //     let p3 = t.0 == u.0 && t.1 != u.1 && t.2 == u.2;

            //     if p1 || p2 || p3 {
            //         output += n_t * n_u; // blunder: +, max, then *
            //         seen.insert((t, u));
            //     }
            // }
        }
        outputs.push(output);

        // idea 2: sort? how would i sort?
        // -- this doesn't seem promising
        // -- sorting, and comparing neighbors is R^1
        // -- when pairwise comparison is combinatorial R^n
        // 1 2 3
        // 1 2 4
        // 1 2 5

        //---
        //---
        //     v
        // 1 2 6
        // 1 2 7
        // 1 2 8
        //---

        //   v
        // 1 1 3
        // 2 2 4 <-------these would be interleaved with the block above
        // 2 3 5

        // v
        // 3 2 3
        // 3 2 4
        // 3 2 5

        // idea 3: analytical sol?
        // --can't access hashmap with invariants

        // for t in triples:
        //   triples.get((.0!=.0), (.1=.1), (.2=.2))

        // idea 4:
        // --is there a notion of sort for more than one dimension? vector space?
        // for t in triples:
        //   neighbors = foo(t)
        //   for n in neighbors {}

        // build a three hashmaps, one for each dim

        // [1, 2, 3]        [1, 2, 3]
        // [2, 3, 2]        [2, 3, 2]
        // [3, 2, 2]        [3, 2, 2]
        // [2, 2, 3]        [2, 2, 3]
        // [2, 3, 4]        [2, 3, 4]
        // [3, 4, 2]        [3, 4, 2]

        // 1 2 3
        // 1 2 4
        // 1 2 5

        //---
        //---
        //     v
        // 1 2 6
        // 1 2 7
        // 1 2 8
        //---

        //   v
        // 1 1 3
        // 2 2 4 <-------these would be interleaved with the block above
        // 2 3 5

        // v
        // 3 2 3
        // 3 2 4
        // 3 2 5
    }

    for o in outputs {
        println!("{o}");
    }
}
