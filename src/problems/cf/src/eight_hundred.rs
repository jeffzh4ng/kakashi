use std::{io, mem};

pub fn solve_1773_f() -> Result<(), io::Error> {
    let mut line_one = String::new();
    let mut line_two = String::new();
    let mut line_three = String::new();

    io::stdin().read_line(&mut line_one)?;
    io::stdin().read_line(&mut line_two)?;
    io::stdin().read_line(&mut line_three)?;

    let n = line_one.trim().parse::<i32>().unwrap();
    let mut a = line_two.trim().parse::<i32>().unwrap();
    let mut b = line_three.trim().parse::<i32>().unwrap();
    let mut swapped = false;
    if b < a {
        mem::swap(&mut a, &mut b);
        swapped = true
    }

    let mut matches = vec![(0, 0); n as usize];

    // spread the smaller goals
    for i in 0..a {
        let index = if i >= n {
            let len = matches.len();
            i as usize % len
        } else {
            i as usize
        };

        let prev = matches[index].0;
        matches[index] = (prev + 1, 0);
    }

    // create wins with the larger goals, starting at the back
    let mut matches_rev = matches.into_iter().rev().collect::<Vec<_>>();
    // for i in 0..b {
    let mut i: usize = 0;
    let mut prev_tie_i = None;
    while b > 0 {
        let prev = matches_rev[i].1;

        if prev + 1 == matches_rev[i].0 {
            if b >= 2 {
                // leapfrog the tie and win the game via +2
                matches_rev[i] = (matches_rev[i].0, prev + 2);
                b -= 2;
                prev_tie_i = None
            } else if prev_tie_i.is_some_and(|index| index == i) {
                // forced to tie
                matches_rev[i] = (matches_rev[i].0, prev + 1);
                b -= 1;
            } else {
                prev_tie_i = Some(i);
            }

            // when are we forced to make a tie vs skip?
        } else {
            // continue incrementing b's goals by 1
            matches_rev[i] = (matches_rev[i].0, prev + 1);
            b -= 1;
            prev_tie_i = None;
        }

        i = (i + 1) % n as usize;
    }

    let ties = matches_rev
        .iter()
        .fold(0, |acc, mtch| if mtch.0 == mtch.1 { acc + 1 } else { acc });

    println!("{ties}");
    for mtch in matches_rev {
        let a = mtch.0;
        let b = mtch.1;
        if swapped {
            println!("{b}:{a}");
        } else {
            println!("{a}:{b}");
        }
    }
    // println!("{:?}", matches_rev.iter().rev().collect::<Vec<_>>());

    Ok(())
}

// eg.1
// n:3              1:0 W <--- skip the tie
// a:2              1:2 W
// b:4              0:2 W

// eg.2

// n:1              2:2 <--- forced to tie
// a:2
// b:2

// obs 1: minimize draws <==> maximizing wins

// idea 1: greedily spread out goals one by one
//         - example above is a counter ex
//         - allocating b's goals in the first match ==> one draw (last match 0:0)
//         - so what we start allocating b by beating the matches with least goals from a
//         - how do we allocate two goals? at once, or via round robin fashion?
