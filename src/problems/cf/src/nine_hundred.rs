use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    io,
};

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

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct PersuasionJobTuple((i32, i32));

impl Ord for PersuasionJobTuple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub fn solve_1089_l() -> Result<(), io::Error> {
    let mut line_one = String::new();
    let mut line_two = String::new();
    let mut line_three = String::new();

    io::stdin().read_line(&mut line_one).unwrap();
    io::stdin().read_line(&mut line_two).unwrap();
    io::stdin().read_line(&mut line_three).unwrap();

    let nk = line_one
        .trim()
        .split(" ")
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let _ = nk[0];
    let k = nk[1];

    let jobs = line_two
        .trim()
        .split(" ")
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let persuasions = line_three
        .trim()
        .split(" ")
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut fulfilled_jobs = HashSet::new();
    let mut fulfilled_jobs_first_index: HashMap<i32, usize> = HashMap::new();
    let mut overfilled_jobs_to_persuasions: HashMap<i32, BinaryHeap<Reverse<i32>>> = HashMap::new();
    let mut overfilled_persuasions = BinaryHeap::new();

    for (i, j) in jobs.iter().enumerate() {
        if fulfilled_jobs.contains(&j) {
            overfilled_jobs_to_persuasions
                .entry(*j)
                .and_modify(|ofp: &mut BinaryHeap<Reverse<i32>>| {
                    ofp.push(Reverse(persuasions[i]));
                })
                .or_insert({
                    let mut ofp = BinaryHeap::new();
                    ofp.push(Reverse(persuasions[i]));
                    if fulfilled_jobs_first_index.contains_key(j) {
                        ofp.push(Reverse(
                            persuasions[*fulfilled_jobs_first_index.get(j).unwrap()],
                        ));
                    }

                    ofp
                });
            overfilled_persuasions.push(Reverse(PersuasionJobTuple((persuasions[i], *j))));
            if fulfilled_jobs_first_index.contains_key(j) {
                overfilled_persuasions.push(Reverse(PersuasionJobTuple((
                    persuasions[*fulfilled_jobs_first_index.get(j).unwrap()],
                    *j,
                ))));
                fulfilled_jobs_first_index.remove(j);
            }
            // println!(
            //     "inserting overfilled p {} for j {j}",
            //     persuasions[*fulfilled_jobs_first_index.get(j).unwrap()]
            // )
        } else {
            fulfilled_jobs.insert(j);
            fulfilled_jobs_first_index.insert(*j, i);
        }
    }

    let unfulfilled_jobs =
        HashSet::<i32>::from_iter((1..=k).into_iter().filter(|x| !fulfilled_jobs.contains(x)));

    // problem: we are underreporting for test case #17.
    //   - is overfilled_persuasions correctly ordered?
    //   - are we leaving in persuasions?

    // if overfilled_persuasions is correctly ordered, which is
    // most plausible since it's a min heap and we are *under*reporting,
    // then we are most likely not reaching higher cost persuasions
    // when we should be.

    // ==>something is wrong with the removal logic..

    // cost = 0
    // for uj in uj:
    //   (p,ofj) = overfilled_persuasions.pop()
    //   cost += p
    //   ofj_ofp[ofj].remove(p)
    //   if ofj_ofp[j].len() == 1:
    //     overfilled_persuasions.remove_once(ofj_ofp[j].pop())

    // 12 9
    // 6 8 2 6 8 2 6 6 6 5 1 1
    // 59 27 78 92 31 26 59 12 80 77 94 31
    // unfulfilled jobs: {3, 9, 7, 4}
    // overfilled persuasions:

    // 12+26+27+31=96
    // [Reverse((12, 6)), 3
    // Reverse((26, 2)), 9
    // Reverse((59, 6)),
    // Reverse((27, 8)), 7
    // Reverse((31, 1)), 4
    // Reverse((78, 2)), <----- cancel
    // Reverse((59, 6)),
    // Reverse((92, 6)),
    // Reverse((80, 6)),
    // Reverse((31, 8)), <----- cancel
    // Reverse((94, 1))] <----- cancel

    let mut output = 0;
    for _ in unfulfilled_jobs {
        if let Some(Reverse(PersuasionJobTuple((p, ofj)))) = overfilled_persuasions.pop() {
            // println!("persuading: {:?}", p);
            output += p;

            overfilled_jobs_to_persuasions
                .entry(ofj)
                .and_modify(|ofps| {
                    // remove p from ofj->ofp
                    let mut removed = false;
                    ofps.retain(|Reverse(ofp)| {
                        if !removed && *ofp == p {
                            removed = true;
                            return false;
                        } else {
                            return true;
                        }
                    });

                    // if the overfilled job is no longer overfilled, (check via overfilled_jobs_to_persuasions)
                    // remove overfilled persuasion (remove via overfilled_persuasions heap)
                    if ofps.len() == 1 {
                        let persuasion_no_longer_overfilled = ofps.pop().unwrap().0;
                        let mut removed = false;
                        overfilled_persuasions.retain(|Reverse(PersuasionJobTuple((ofp, _)))| {
                            if !removed && *ofp == persuasion_no_longer_overfilled {
                                // println!("goodbye persuasion {p}, bc we used up job {j}");
                                removed = true;
                                return false;
                            } else {
                                return true;
                            }
                        })
                    }
                });
        } else {
            panic!() // todo
        }
    }

    println!("{output}");
    Ok(())
}

// editorial: For each job, find the idler with the maximum value of bi, assign
//            this job to this idler. Pull all remaining idlers in the array,
//            sort them by the value of bi and assign idlers with *minimum* values
//            to remaining jobs.

// notes: when having to build complicated state, try doing the converse
//        i could have assigned idlers to jobs first, so then i would never
//        have to make any updates to persuasions no longer overfilled.

// still, the assigning part of the problem is taking the minimum.
// how am i underreporting for test case 17?

// overfilled persuasions:
// [
// Reverse(PersuasionJobTuple((12, 6))),
// Reverse(PersuasionJobTuple((26, 2))),
// Reverse(PersuasionJobTuple((59, 6))),
// Reverse(PersuasionJobTuple((27, 8))),
// Reverse(PersuasionJobTuple((31, 1))),
// Reverse(PersuasionJobTuple((78, 2))),
// Reverse(PersuasionJobTuple((59, 6))),
// Reverse(PersuasionJobTuple((92, 6))),
// Reverse(PersuasionJobTuple((80, 6))),
// Reverse(PersuasionJobTuple((31, 8))),
// Reverse(PersuasionJobTuple((94, 1)))]
