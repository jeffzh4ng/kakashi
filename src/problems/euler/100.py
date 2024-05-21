import functools


def solve_one():
    return functools.reduce(
        lambda x, y: x+y,
        filter(lambda x: x % 3 == 0 or x % 5 == 0, range(1000))
    )
