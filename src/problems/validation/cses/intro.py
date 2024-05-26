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

    # 8 possible choices for second k2
    k2_attack_vectors = [
        [-2, -1], [-2, 1],
        [-1, -2], [-1, 2],
        [1, -2], [1, 2],
        [2, -1], [2, 1]
    ]

    output = []
    for k in range(2, n+1):
        candidates = factorial(k*k)//(factorial(2) * (factorial((k*k)-2)))
        grid = [[0 for _ in range(k)] for _ in range(k)]
        # print(candidates)

        legal_k2_attacks = set()
        for i in range(len(grid)):
            for j in range(len(grid[i])):
                for coord in k2_attack_vectors:
                    if (i + coord[0] >= 0 and i + coord[0] < len(grid)) and (j + coord[1] >= 0 and j + coord[1] < len(grid[i])):
                        if i+j < ((i+coord[0])+(j+coord[1])):
                            legal_k2_attacks.add(
                                f"({i},{j})-({i+coord[0]},{j+coord[1]})")
                        else:
                            legal_k2_attacks.add(
                                f"({i+coord[0]},{j+coord[1]})-({i},{j})")

        output.append(candidates-(len(legal_k2_attacks)))

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
    inp = [i for i in range(1, n+1)]

    a = set()
    b = set()

    for x in reversed(inp):
        sum_a = reduce(lambda x, y: x+y, a, 0)
        sum_b = reduce(lambda x, y: x+y, b, 0)

        if sum_a < sum_b:
            a.add(x)
        else:
            b.add(x)

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


# solve_eight()


# idea 1: gen all perms with splits, check if splits have same sum --> prob TLE
# idea 2: use even/odd property?
# idea 3: use some analytical formula? sum = n(n+1)/2
# idea 4: greedy: iterate 1..n, place elements in the set with less sum
# idea 5: greedy: iterate n..1, place elements in the set with les sum-->TLE
