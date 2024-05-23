from sys import stdin


def solve_weird_algorithm():
    n = int(input())
    output = [n]

    while output[-1] != 1:
        if output[-1] % 2 == 0:
            output.append(output[-1] // 2)
        else:
            output.append(output[-1]*3+1)

    return " ".join(map(str, output))
