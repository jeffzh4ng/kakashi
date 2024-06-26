from sys import stdin
from functools import reduce
from math import factorial


def solve_one():
    n = int(input())
    output = [n]

    while output[-1] != 1:
        if output[-1] % 2 == 0:
            output.append(output[-1] // 2)
        else:
            output.append(output[-1]*3+1)

    return " ".join(map(str, output))


def solve_two():
    n = int(input())
    inp = map(int, input().split(" "))
    output = reduce(lambda x, y: x - y, inp, (n*(n+1))//2)

    return output


def solve_three():
    inp = input()

    max_count = 1
    cur_count = 1
    for i in range(1, len(inp)):
        if inp[i] == inp[i-1]:
            cur_count += 1
        else:
            max_count = max(max_count, cur_count)
            cur_count = 1

    max_count = max(max_count, cur_count)
    return max_count


def solve_four():
    n = int(input())
    inp = list(map(int, input().split(" ")))
    outp = 0

    for i in range(len(inp)-1):
        if inp[i] > inp[i+1]:
            outp += inp[i] - inp[i+1]
            inp[i+1] = inp[i]

    return outp


# idea 1:
# - walk through the list?
#   - i: how would i know how to update i??
#   - can i somehow update just based off a neighbor update?
#   - lowering i to i+1 heuristic does not work
#   counter:
#     6 10 4 10 2 8 9 2 7 7
#     6 4 4 10 2 8 9 2 7 7 <---- fail. 10 is lowered down to 4 which breaks monotonicity

# can we reverse the heuristic to increasing in order to maintain the monotonic invariant?


# idea 2:
# - is there an analytic formula here?
# - what if i sum the numbers
# - that's going to lose all the information though
# - bad idea

def solve_five():
    n = int(input())
    if n == 1:
        return str(n)
    elif n == 2 or n == 3:
        return "NO SOLUTION"
    else:
        output = []

        for i in [i for i in range(2, n+1, 2)]:
            output.append(i)
        for i in [i for i in range(1, n+1, 2)]:
            output.append(i)

        return " ".join(map(str, output))

# exs
# - solve_five(5) ==> 4 2 5 3 1
# - solve_five(3) ==> "NO SOLUTION"

# note: there might be sev solutions, so solve_five is a rel not a fun

# idea 1:
# - filter(lambda x: beautiful(x), gen_perms())
#  - gen_perms is going to take O(n!) though ==> TLE
#  - can we generate perms in a quicker way?

# idea 2:
# - is there some way to selectively pick the answer at each i?
#   - is there some analytic solution here?
#   - pick the middle, and then oscillate high and low vals?

# idea 3:
# - selectively pick each answer globally, not at each i.
#   - place the evens, then the odds

# 2 3 -> 8
# 1 1 -> 1
# 4 2 -> 15

# https://www.youtube.com/watch?v=KOJTK0W1CzA&list=PLjyTwPYGTd7QOG1jLIIs_RkYkFJpj27h4&index=5
# https://www.youtube.com/watch?v=6B5xEU8-jaA&list=PLjyTwPYGTd7QOG1jLIIs_RkYkFJpj27h4&index=7


def solve_six():
    n = int(input())
    inp = []
    for _ in range(n):
        tmp = list(map(lambda x: int(x), input().split(" ")))
        inp.append(tmp)

    print(inp)
    output = []

    for c in inp:
        if c[1] > c[0]:
            base = (c[1]-1)*(c[1]-1)  # inner sq
            print("base", base)

            if c[1] % 2 == 0:
                offset = c[0]  # offset is before half
                output.append(base + offset)
            else:
                offset = (2*c[1])-c[0]  # offset is after half
                output.append(base + offset)

        else:
            base = (c[0]-1)*(c[0]-1)  # inner sq
            print("base", base)

            if c[0] % 2 == 0:
                offset = (2*c[0])-c[1]  # offset is after half
                output.append(base + offset)
            else:
                offset = c[1]  # offset is before half
                output.append(base + offset)

    return output

# idea 1: generate number spiral through enumeration --> TLE
# idea 2: smaller generation space: generate pivots --> TLE
# idea 3: O(logn)? can we divide the search space by half?
# idea 4: analytical sol? yes, the analytical sol involves x^2. NOT pivots.

# ----------(2,3) -> 8
# ----------(1,1) -> 1
# ----------(4,2) -> 15


def solve_seven():
    n = int(input())
    output = []
    for k in range(2, n+1):
        cands = (k*k)*(k*k-1)/2
        legal_k2_attacks = (8*(k-1)*(k-2))/2

        output.append(cands-legal_k2_attacks)

    return output


# for x in solve_seven():
#     print(x)

# idea 1: (#ways to place 2 knights)-(#ways to place 2 attacking nights)

# k = 1 -------> (1 choose 2) = 0
# 0

# k = 2
# 00 -----> (4 choose 2) = (4*3*2*1)/2*(4-2)!
# 00                     = 24/4 = 6 (24 perms, remove the double counts of 0s/1s)

# k = 3
# 000------> (9 choose 2) = 9!/2!(7!)
# 000                     = 36 - |illegal board states|(8)
# 000                     = 28


# k = 4 -----> (25 choose 2) = 300 - |illegal board states|(204)
#                            = 96

# is there a way to analytically count #illegal board states without enumeration?
# --> is there a way to count #illegal board states with step rule?
# --> first knight: n! choices
# --> second knight:


#  0 K 0 K 0
#  K 0 0 0 K
#  0 0 k 0 0
#  K 0 0 0 K
#  0 K 0 K 0

# 8 possible choices for second K, assuming k is placed in center
# k2_vectors = [
#     [-2, -1], [-2, 1],
#     [-1, -2], [-1, 2],
#     [1, -2], [1, 2],
#     [2, -1], [2, 1]
# ]

# counting legal_k2 attacks through k1(i,j) enumeration ---> TLE


# idea 2: same, but count second term analytically, not through k1(i,j) enumeration

def solve_eight():
    n = int(input())
    if ((n*(n+1))//2) % 2 != 0:
        print("NO")
        return

    inp = [i for i in range(1, n+1)]

    a = set()
    b = set()
    sum_a = 0
    sum_b = 0

    for x in reversed(inp):
        if sum_a < sum_b:
            a.add(x)
            sum_a += x
        else:
            b.add(x)
            sum_b += x

    sum_a = reduce(lambda x, y: x+y, a, 0)
    sum_b = reduce(lambda x, y: x+y, b, 0)

    if sum_a == sum_b:
        print("YES")
        print(len(a))
        print(" ".join(map(str, list(a))))
        print(len(b))
        print(" ".join(map(str, list(b))))
    else:
        print("NO")


# idea 1: gen all perms with splits, check if splits have same sum --> prob TLE
# idea 2: use even/odd property?
# idea 3: use some analytical formula? sum = n(n+1)/2
# idea 4: greedy: iterate 1..n, place elements in the set with less sum
# idea 5: greedy: iterate n..1, place elements in the set with les sum-->TLE

def solve_nine():
    n = int(input())
    return pow(2, n) % 1000000007

# 20! = 20 * 10


def solve_ten():
    n = int(input())
    n_fac = str(factorial(n))

    i = len(n_fac)-1
    count = 0
    while n_fac[i] == "0":
        count += 1
        i -= 1

    return count


# idea 1: built in factorial --> TLE
# idea 2:


def solve_eleven():
    n = int(input())
    inp = []
    for _ in range(n):
        inp.append(list(map(int, input().split(" "))))

    output = []
    for i in inp:
        left = i[0]
        right = i[1]

        if (right == left*2) or (left == right*2):
            left = 0
            right = 0

        while left > 0 and right > 0:
            if left < right:
                left -= 1
                right -= 2
            else:
                right -= 1
                left -= 2

        if left == 0 and right == 0:
            output.append("YES")
        else:
            output.append("NO")

    return output

# idea 1: heuristic--take 1 off the side with the last amount, 2 off the other --> TLE
# idea 2 heuristic--but take larger steps to 0?
# idea 3: analytic--skip when a=2*b or b=2*a
# idea 4: analytic--even/odd?
# idea 5: analytic--difference? (a-b)


def solve_twelve():
    s = input()
    char_counts = reduce(
        lambda m, c: {**m, c: m.get(c, 0) + 1}, list(s), dict())

    odd_count = 0
    odd_c = ""
    for k, v in char_counts.items():
        if v % 2 != 0:
            odd_count += 1
            odd_c = k*v

    output = odd_c
    if odd_count > 1:
        print("NO SOLUTION")
        return
    else:
        for k, v in char_counts.items():
            if v % 2 != 0:
                continue

            count = v
            while count > 0:
                output = k + output + k
                count -= 2

    print(output)


solve_twelve()

# idea 1: hashmap -> TLE
# idea 2: reorder through swaps?


# AAAACACBA
# AACABACAA
