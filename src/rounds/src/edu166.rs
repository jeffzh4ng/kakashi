use std::{cmp, io};

pub fn solve_1976a() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();

    for _ in 0..n {
        let mut line_one = String::new();
        let mut line_two = String::new();

        io::stdin().read_line(&mut line_one).unwrap();
        io::stdin().read_line(&mut line_two).unwrap();

        input.push(line_two)
    }

    for p in input {
        let mut seen_letter = false;
        let mut no_digit_after_letter = true;

        for c in p.chars() {
            if c.is_numeric() && seen_letter {
                no_digit_after_letter = false;
            } else if c.is_alphabetic() {
                seen_letter = true;
            }
        }

        let mut digit_non_decreasing = true;
        for (i, c) in p.chars().enumerate() {
            if c.is_numeric() && i > 0 {
                if p.as_bytes()[i - 1] as i32 > p.as_bytes()[i] as i32 {
                    digit_non_decreasing = false;
                }
            }
        }

        let mut letter_non_decreasing = true;
        for (i, c) in p.chars().enumerate() {
            if c.is_alphabetic() && i > 0 {
                if (p.as_bytes()[i - 1] as char)
                    .cmp(&(p.as_bytes()[i] as char))
                    .is_gt()
                {
                    letter_non_decreasing = false;
                }
            }
        }

        let valid = no_digit_after_letter && digit_non_decreasing && letter_non_decreasing;
        if valid {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

pub fn solve_1976_b() {
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

        let _ = line_one.trim().parse::<i32>().unwrap();
        let input_a = line_two
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let input_b = line_three
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((input_a, input_b));
    }

    let mut output = Vec::new();

    for (input_a, input_b) in input {
        let mut operations = 0;

        let mut input_a_last_element = false;
        let input_b_last_element = *(input_b.iter().rev().take(1).collect::<Vec<_>>()[0]);

        let mut max_ceil = 0;
        let mut min_floor = 0;

        // update a0..an
        for i in 0..input_a.len() {
            if input_a[i] != input_b[i] {
                // idea 1: ==> O(n^2) ==> TLE for 1e9 test cases
                // for a in A
                //   while loop for updating ai
                //     if ai is bn, use a copy

                // idea 2: analytical?
                // - when is b_n NOT contained in the difference?
                // - case 1: dec ai to bi, and (b_n > ai or b_n < bi)
                // - case 2: inc ai to bi

                // inc or dec
                let diff = (input_a[i] - input_b[i]).abs();
                operations += diff;

                let ceil = cmp::max(input_a[i], input_b[i]);
                let floor = cmp::min(input_a[i], input_b[i]);

                max_ceil = cmp::max(max_ceil, ceil);
                min_floor = cmp::min(min_floor, floor);

                // copy if last_element is within A[i] - B[i]
                if (input_b_last_element <= ceil && input_b_last_element >= floor)
                    && !input_a_last_element
                {
                    operations += 1;
                    input_a_last_element = true;
                }
            } else {
                // A[i] = B[i]
                // so if A[i] == B[-1], we can copy
                if input_a[0] == input_b_last_element && !input_a_last_element {
                    operations += 1;
                    input_a_last_element = true;
                }
            }
        }

        // 1
        // 2
        // 1 3

        // 2
        // 2 2  <-- 1
        // 1 2  <-- 2
        // 1 3  <-- 3

        // when is this being hit?
        // ==>when B_las_element is in between
        // ==>but how is this the case when:
        //   - pred: B_last_element <= ceil && B_last_element >= floor
        //   - failed for all i?

        // 1
        // 2
        // 10
        // 1 11
        //

        // last element not set ==> B[-1] was never within ceil-floor range
        if !input_a_last_element {
            // take the difference from the closest ceil or floor to B_last_element
            // --find if B_last_element is greater than max_ceil or less than min_floor

            let diff = if input_b_last_element >= max_ceil {
                (max_ceil - input_b_last_element).abs()
            } else if input_b_last_element <= min_floor {
                (min_floor - input_b_last_element).abs()
                // else if B_last_element < min_floor {
            } else {
                // when is this being hit?
                // ==>when B_las_element is in between
                // ==>but how is this the case when:
                //   - pred: B_last_element <= ceil && B_last_element >= floor
                //   - failed for all i?
                panic!()
            };
            operations += diff + 1; // + 1 for the copy
        }

        output.push(operations);
    }

    for o in output {
        println!("{o}");
    }
}

//(B_last_element <= ceil && B_last_element >= floor) 3
// 1
// 2
// 1 3

// tc2
// 2
// 3 3 ========================> 1
// 3 3 3

// tc3
// 4
// 4 2 1 2 =====================> 8
// 2 1 5 2 3

// 4 2 1 2
// 3 2 1 2
// 3 2 1 2 3

// 2 2 1 2 3 <-- 0

// 2 1 1 2 3 <-- 1

// 2 1 2 2 3 <-- 2
// 2 1 3 2 3
// 2 1 4 2 3
// 2 1 5 2 3

// tc4
// 2
// 3 3 ========================> 1
// 3 3 4

// idea 1: you want to always copy the last element in the array

pub enum Skill {
    P,
    T,
}

pub fn solve_1976_c() {
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

        let nm = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let input_a = line_two
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let input_b = line_three
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((nm, input_a, input_b));
    }

    let mut output = Vec::new();

    // p1 t2
    // p:2 1 5 4
    // t:5 2 3 1

    for (pt, p_skills, t_skills) in input {
        let (p, t) = (pt[0], pt[1]);
        let mut team_skill_less_ith_cand = Vec::new();

        // idea 1: double for loop?
        // idea 2: rebuild p and t for efficient processing
        //           - but we don't simply sum all the p
        //           - we selectively choose

        // p1 t2
        // p: 2    1   *5*  *4*
        // t:*5*   *2*  3   1
        // c: (t,i,5) (t,i,2)   (p,i,5)  (p,i,4)

        // p1   t0
        // p:*2* 1
        // t:1  *2*

        let mut cands_better_skill = Vec::new();

        for i in 0..p_skills.len() {
            if p_skills[i] > t_skills[i] {
                cands_better_skill.push((Skill::P, i));
            } else {
                cands_better_skill.push((Skill::T, i));
            }
        }

        for i_to_omit in 0..cands_better_skill.len() {
            let (mut p, mut t) = (p, t);
            let mut hired_team_skill_without_i = 0;

            for j in 0..cands_better_skill.len() {
                if i_to_omit == j {
                    continue;
                } else {
                    match cands_better_skill[j].0 {
                        Skill::P => {
                            if p > 0 {
                                hired_team_skill_without_i += p_skills[cands_better_skill[j].1];
                                p -= 1;
                            } else {
                                hired_team_skill_without_i += t_skills[cands_better_skill[j].1];
                                t -= 1;
                            }
                        }
                        Skill::T => {
                            if t > 0 {
                                hired_team_skill_without_i += t_skills[cands_better_skill[j].1];
                                t -= 1;
                            } else {
                                hired_team_skill_without_i += p_skills[cands_better_skill[j].1];
                                p -= 1;
                            }
                        }
                    }
                }
            }

            team_skill_less_ith_cand.push(hired_team_skill_without_i);
        }

        output.push(team_skill_less_ith_cand);
    }

    for o in output {
        // println!("{:?}", o.join(" ").collect::<String>());
        for i in o {
            print!(" {:?}", i);
        }
        print!("\n")
        // println!("");
    }
}

//(B_last_element <= ceil && B_last_element >= floor) 3
// 1
// 2
// 1 3

// tc2
// 2
// 3 3 ========================> 1
// 3 3 3

// tc3
// 4
// 4 2 1 2 =====================> 8
// 2 1 5 2 3

// 4 2 1 2
// 3 2 1 2
// 3 2 1 2 3

// 2 2 1 2 3 <-- 0

// 2 1 1 2 3 <-- 1

// 2 1 2 2 3 <-- 2
// 2 1 3 2 3
// 2 1 4 2 3
// 2 1 5 2 3

// tc4
// 2
// 3 3 ========================> 1
// 3 3 4

// idea 1: you want to always copy the last element in the array
