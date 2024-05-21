import functools


def solve_one():
    return functools.reduce(
        lambda x, y: x+y,
        filter(lambda x: x % 3 == 0 or x % 5 == 0, range(1000))
    )


def solve_two():
    x = 1
    y = 2
    sum = 2

    while (y < 4e6):
        tmp = y
        y = x+y
        x = tmp
        print(y)

        if y % 2 == 0:
            sum += y

    return sum
