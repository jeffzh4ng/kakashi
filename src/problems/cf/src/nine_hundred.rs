use std::io;

pub fn solve_1267_b() -> Result<(), io::Error> {
    fn segments(inp: &str) -> Vec<(usize, usize)> {
        let inp = inp.chars().collect::<Vec<_>>();
        let mut output = Vec::new();

        let mut l = 0;
        let mut r = 0;
        while l < inp.len() && r < inp.len() {
            while r < inp.len() && inp[r] == inp[l] {
                r += 1
            }

            // segment must at least be length 2,
            // so the additional char makes a segment
            // of length 3
            let segment_at_least_length_two = (r - l) >= 2;

            // forming elongated segments by
            // eliminating side segments is
            // not possible.
            let not_side_segment = l != 0 && r - 1 != inp.len() - 1;
            if segment_at_least_length_two && not_side_segment {
                output.push((l, r - 1));
            }
            l = r
        }

        output
    }

    fn eliminate(inp: String, l: usize, r: usize) -> Option<String> {
        if inp.chars().nth(l) != inp.chars().nth(r) {
            None
        } else {
            let left = inp.chars().take(l);
            let right = inp
                .chars()
                .rev()
                .take(inp.len() - 1 - r)
                .collect::<Vec<_>>()
                .into_iter()
                .rev();
            let output = left.chain(right);

            Some(output.collect())
        }
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let tmp = input.trim();

    let mut output = 0;
    for (l_start, r_start) in segments(tmp) {
        let (mut l, mut r) = (l_start, r_start);
        let mut cand = tmp.chars().collect::<Vec<_>>();
        let choices = r - l + 2;

        while cand.len() > 0 {
            if let Some(cand_eliminated) = eliminate(cand.iter().collect(), l, r) {
                // println!(
                //     "{:?} ->eliminate-> {:?}",
                //     cand.iter().collect::<String>(),
                //     cand_eliminated
                // );
                cand = cand_eliminated.chars().collect::<Vec<_>>();
                if cand.len() == 0 {
                    break;
                }

                // reset l and r
                r = l;
                if l > 0 {
                    l = l - 1;
                }

                if cand[l] != cand[r] {
                    break; // no segment was elongated
                } else {
                    // set l and r to the elongated segment
                    while r < cand.len() - 1 && cand[r] == cand[r + 1] {
                        r += 1;
                    }
                    while l > 0 && cand[l] == cand[l - 1] {
                        l -= 1;
                    }

                    if r - l + 1 < 3 {
                        // no segment was elongated by at least 3
                        break;
                    }
                }
            } else {
                break; // cand cannot be eliminated at l and r
            }
        }

        if cand.len() == 0 {
            // println!("{tmp} successfully popped starting at {l_start},{r_start}",);
            output += choices;
        }
    }

    println!("{output}");

    Ok(())
}

// how do we keep track of segment lengths?

// idea 1: two pointers?
// idea 2:
