use std::io;

pub fn solve_a() -> Result<(), io::Error> {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        let mut line_two = String::new();

        io::stdin().read_line(&mut line_one).unwrap();
        io::stdin().read_line(&mut line_two).unwrap();

        let nm = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (n, m) = (nm[0], nm[1]);

        let input_a = line_two.trim().chars().collect::<Vec<_>>();

        input.push((n, m as u32, input_a))
    }

    let mut output = Vec::new();

    for tc in input {
        let (n, m, input_a) = tc;

        // char_count
        let mut char_count = std::collections::HashMap::new();
        for c in &input_a {
            // println!("bar: {:?}{:?}", input_a, c);
            let prev: u32 = *char_count.entry(c).or_default();
            char_count.insert(c, prev + 1);
        }

        let mut o = 0;
        for (_, v) in &char_count {
            if v < &m {
                o += m as u32 - v;
            }
        }
        // println!("moose:{o}");

        // for every non-existen characters
        // println!("wolf{:?}, {:?}", char_count, char_count.len());
        let non_existent_chars = 7 - char_count.len();
        if non_existent_chars > 0 {
            o += m * non_existent_chars as u32;
        }

        output.push(o);
    }

    for o in output {
        println!("{o}");
    }

    Ok(())
}

pub fn solve_b() -> Result<(), io::Error> {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        let mut line_two = String::new();

        io::stdin().read_line(&mut line_one).unwrap();
        io::stdin().read_line(&mut line_two).unwrap();

        let nfk = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (n, f, k) = (nfk[0], nfk[1], nfk[2]);

        let input_a = line_two
            .trim()
            .chars()
            .filter(|c| *c != ' ')
            .collect::<Vec<_>>();

        input.push((n, f, k, input_a))
    }

    let mut output = Vec::new();
    for tc in input {
        let (n, f, k, mut input_a) = tc;

        let fav = input_a[f - 1];
        input_a.sort();
        input_a.reverse();

        let mut remove_fav = false;
        let mut remove_last_fav = false;
        for i in 0..k {
            if input_a[i] == fav {
                remove_fav = true;

                if (i + 1 < input_a.len() && input_a[i] != input_a[i + 1])
                    || (i + 1 == input_a.len())
                {
                    remove_last_fav = true;
                }
            }
        }

        let o = match (remove_fav, remove_last_fav) {
            (true, true) => "YES",
            (true, false) => "MAYBE",
            (false, true) => panic!(),
            (false, false) => "NO",
        };
        output.push(o);
    }

    for o in output {
        println!("{o}");
    }

    Ok(())
}

// All operations must be applied to the array in the given order.
// More than one operation can be applied to a single position.
pub fn solve_c() -> Result<(), io::Error> {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        let mut line_two = String::new();
        let mut line_three = String::new();
        let mut line_four = String::new();
        let mut line_five = String::new();

        io::stdin().read_line(&mut line_one).unwrap();
        io::stdin().read_line(&mut line_two).unwrap();
        io::stdin().read_line(&mut line_three).unwrap();
        io::stdin().read_line(&mut line_four).unwrap();
        io::stdin().read_line(&mut line_five).unwrap();

        // let nfk = line_one
        //     .trim()
        //     .split(" ")
        //     .map(|c| c.parse::<usize>().unwrap())
        //     .collect::<Vec<_>>();
        // let (n, f, k) = (nfk[0], nfk[1], nfk[2]);

        // blunder, forgetting .join syntax
        let input_a = line_two
            .trim()
            .chars()
            .filter(|c| *c != ' ')
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let input_b = line_three
            .trim()
            .chars()
            .filter(|c| *c != ' ')
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let input_d = line_five
            .trim()
            .chars()
            .filter(|c| *c != ' ')
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // input.push((n, f, k, input_a))

        input.push((input_a, input_b, input_d))
    }

    // let mut output = Vec::new();
    for tc in input {
        let (input_a, input_b, input_d) = tc;

        // is the a sequence of ijs st a===>(ij_vec)===> b

        // -the indices of c don't matter
        // -if b is a subset d return true??

        // how a and c matter?
        // b is the codomain
        // d is the mapping f(a)

        // does there exist f(a)<-{d} st. d: a -> b

        // can i provide a counter example where b

        // if there is no covering up to do (b = d) "YES"
        // if there is covering up to do
        //   b is a subset of d but d is not a subset of b

        // blunder: sets, vs maps
        // didn't realize i needed to *count*

        // blunder with hashset types
        // let set_b: std::collections::HashSet<i32> =
        //     std::collections::HashSet::from_iter(input_b.iter());
        // let set_d: std::collections::HashSet<i32> =
        //     std::collections::HashSet::from_iter(input_d.iter());

        let mut map_b = std::collections::HashMap::new();
        let mut map_d = std::collections::HashMap::new();
        for b in input_b {
            let prev: u32 = *map_b.entry(b).or_default();
            map_b.insert(b, prev + 1);
        }
        for d in input_d.iter().collect::<Vec<_>>() {
            let prev: u32 = *map_d.entry(d).or_default();
            map_d.insert(d, prev + 1);
        }

        // blunder with these hasher types
        let b_keys: std::collections::HashSet<i32> = map_b.keys().cloned().collect();
        // let d_keys: std::collections::HashSet<i32> = map_d.keys().cloned().collect();

        // let o = if !b_keys.is_subset(&d_keys) {
        //     "NO"
        // } else {
        //     // check if the counts are equal
        //     let mut eq_counts = true;
        //     for (b, b_n) in &map_b {
        //         if map_d.get(b).unwrap() < b_n {
        //             eq_counts = false;
        //         }
        //     }

        //     if !eq_counts {
        //         "NO"
        //     } else {
        //         // check if superset of d can be covered up by subset of d

        //         // sequentially walk d

        //         // last index of an element in the superset of d
        //         // if
        //         let mut superset_d = std::collections::HashMap::new();

        //         for d in input_d {
        //             if !b_keys.contains(&&d) {
        //                 let prev: u32 = *superset_d.entry(d).or_default();
        //                 superset_d.insert(d, prev + 1);
        //             } else {
        //                 // we can take one d out of superset_d
        //                 let mut d_remove = 0;
        //                 for (d, _) in &superset_d {
        //                     d_remove = *d;
        //                     break;
        //                 }

        //                 let val = superset_d.get(&&d_remove).unwrap() - 1;
        //                 if val == 0 {
        //                     superset_d.remove(&d_remove);
        //                 } else {
        //                     superset_d.insert(d_remove, val);
        //                 }
        //             }
        //         }

        //         if superset_d.is_empty() {
        //             "NO"
        //         } else {
        //             "YES"
        //         }
        //     }
        // };

        // output.push(o)
    }

    // for o in output {
    //     println!("{o}");
    // }

    Ok(())
}

fn main() {
    solve_c().unwrap();
}
