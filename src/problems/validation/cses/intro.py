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
