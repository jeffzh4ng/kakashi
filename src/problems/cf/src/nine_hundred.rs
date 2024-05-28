use std::io;

pub fn solve_1267_b() -> Result<(), io::Error> {
    fn segments(inp: &str) -> (Vec<(char, usize)>, Vec<usize>) {
        let input = inp.chars().collect::<Vec<_>>();
        let mut input_c = Vec::new();
        let mut segment_indices = Vec::new();

        let mut l = 0;
        let mut r = 0;
        while l < input.len() && r < input.len() {
            while r < input.len() && input[r] == input[l] {
                r += 1
            }

            // segment must at least be length 2,
            // so the additional char makes a segment
            // of length 3
            let segment_at_least_length_two = (r - l) >= 2;

            // forming elongated segments by
            // eliminating side segments is
            // not possible.
            let not_side_segment = l != 0 && r - 1 != input.len() - 1;

            // segments adjacent to current segment must contain
            // the same character in order for the string to be
            // completely eliminated
            let adjacent_segments_same_char =
                not_side_segment && (l >= 1 && r + 1 < input.len()) && input[l - 1] == input[r + 1];

            input_c.push((input[l], r - l));

            if segment_at_least_length_two && not_side_segment && adjacent_segments_same_char {
                segment_indices.push(input_c.len() - 1);
            }
            l = r
        }

        (input_c, segment_indices)
    }

    fn eliminate(mut input: Vec<(char, usize)>, i: usize) -> Vec<(char, usize)> {
        input.remove(i);
        input
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let tmp = input.trim();

    let (input_c, segment_indices) = segments(tmp);
    if input_c.len() % 2 == 0 {
        println!("0");
        return Ok(());
    }

    for mut i in segment_indices {
        let j = i;

        let mut cand = input_c.clone();
        while cand.len() > 0 {
            // println!("cand:{:?}, {i}", cand);
            cand = eliminate(cand, i);
            // println!("cand_eliminated:{:?}", cand);

            let side_segment = i == 0;
            if side_segment {
                break;
            }
            let no_segment_elongated = cand[i - 1].0 != cand[i].0;
            let segment_elongated_less_than_three = cand[i - 1].1 + cand[i].1 < 3;
            if cand.len() == 0 || no_segment_elongated || segment_elongated_less_than_three {
                break;
            } else {
                // merge the left and right segment
                cand[i - 1].1 += cand[i].1;
                cand.remove(i);
                i = i - 1;
            }
        }

        if cand.len() == 0 {
            // println!("{tmp} successfully popped starting at {l_start},{r_start}",);
            println!("{:?}", input_c[j].1 + 1);
            return Ok(());
        }
    }

    println!("0");
    return Ok(());
}

// idea 1: two pointers? -> O(SC) -> O(N^2) -> TLE
// for s in segment:
//   eliminate(s)

// idea 2: how can we skip work?

//  segment:
//  - can we filter out any more segments?
//  - yes, we can filter out segments where input[l-1] != input[r+1]

//   eliminate:
//   - do we need to try eliminating every segment?
//   - do we get any information of surrounding s_(i-1), s_(i+1) with s_(i)?
//   - doesn't seem like it, segments are not guaranteed to be next to each
//     other in the input

// any data structures here useful to store information?

// idea 3: is there another approach here? is two pointers~O(SN) two slow by design?
// - can we get work down to linear?
// - can we use up some space?
//   - would char->counts help here? no.
//   - would char->segment_counts help here?
//     - not sure how you would progress to seeing if the string is completely eliminatable
//     - positional information seems key here

// idea 4: can we do something with two pointer positional information but in one pass?

// hint 5: can we *compress* the string before working with it? -> TLE (still).
// this lifts representation with O(N) so
// the same segment->eliminate strategy
// happens in  O(S) rather than O(SC) < O(N^2)

// problem: the representation level i was working with was completely wrong
//          for the time constraints. characters -> segments

// in: WWWOOOOOOWWW -> in_c: (W:3), (O:6), (W:3)

// TLE:
//   - segments: not filtering enough or
//   - eliminate: not breaking early enough
