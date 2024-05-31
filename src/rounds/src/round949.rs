use std::io;

// A.
// idea1: n^2, prob TLE on 1e9
//  - walk from r to l
//    - walk through all factors

// idea 2: one pass
// - can i avoid walking through all factors?

pub fn solve_r949_a() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        io::stdin().read_line(&mut line_one).unwrap();
        let lr = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((lr[0], lr[1]))
    }

    let mut output = Vec::new();
    for tc in input {
        let (l, r) = tc;

        let mut max_score = 0;
        // choice 1
        for x in l..=r {
            // choice 2
            for p in 2..=(x as f64).sqrt() as i32 {
                // todo: 2..sqrt(x)?
                println!("{x}{p}");
                if p % x == 0 {
                    max_score = std::cmp::max(max_score, p / x);
                }
            }
        }
        println!("{:?}", output);
        output.push(max_score);
    }

    for o in output {
        println!("{o}");
    }
}

// B.
// obs
// - ai = a_i-1 | ai | ai+1
// - is there some structural insight with bitwise OR and aleph_0?
// - when does the sequence converge?

// bitwise or
// - case odd OR odd
// - case even OR even

// - case even OR odd

// 0011:3
// 0111:7
// 0111:7

// 0011:3
// 0101:5
// 0111:7

// 0101:5
// 0111:7
// 0111

// 𝑎0,𝑎1,𝑎2,𝑎3,𝑎4,𝑎5: 0,1,2,3,4,5,6,7,8,9,10
// 𝑎0,𝑎1,𝑎2,𝑎3,𝑎4,𝑎5: 1,3,3,7,7,7
// 𝑎0,𝑎1,𝑎2,𝑎3,𝑎4,𝑎5: 3,3,7,7,7,7

// tc1: fails: undereporting
// e:1279, a:1151
// e:19455, a:19199
pub fn solve_r949_b() {
    let mut line_one = String::new();
    io::stdin().read_line(&mut line_one).unwrap();
    let n = line_one.trim().parse::<i32>().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        let mut line_one = String::new();
        io::stdin().read_line(&mut line_one).unwrap();
        let nm = line_one
            .trim()
            .split(" ")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        input.push((nm[0] as usize, nm[1]))
    }

    let mut output = Vec::new();
    for tc in input {
        let (n, m) = tc;
        let i = n;
        let n = std::cmp::max(n, 1);
        let mut os = (0..=n + 1).into_iter().collect::<Vec<_>>();

        for _ in 0..m {
            // update middle
            let mut ns = os.clone();
            for i in 1..=n {
                ns[i] = os[i - 1] | os[i] | os[i + 1];
            }
            // update start and end
            ns[0] = os[0] | os[1];
            ns[n + 1] = os[n] | os[n + 1];

            // println!("{:?}, {:?}", old_state, new_state);
            os = ns;
        }

        output.push(os[i]);
    }

    for o in output {
        println!("{o}");
    }
}
