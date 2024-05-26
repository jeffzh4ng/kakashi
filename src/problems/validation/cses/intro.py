from sys import stdin
from functools import reduce


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
