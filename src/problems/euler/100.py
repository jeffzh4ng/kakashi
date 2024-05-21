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


def solve_four():
    def palindrome(terms):
        n = str(terms[0]*terms[1])
        i = 0
        j = len(n)-1

        palin = True
        while (i <= j):
            if n[i] != n[j]:
                palin = False
                break
            i += 1
            j -= 1

        return terms[0]*terms[1] if palin else -1

    x = 100
    y = 100

    output = sorted(filter(
        lambda x: x > 0,
        map(palindrome, [(i, j)
                         for i in range(100, 1000) for j in range(100, 1000)])))

    return output[-1]


# revisit with prime factorization
def solve_five():
    n = 21
    while True:
        divides_all = True
        for i in range(1, 21):
            if n % i != 0:
                divides_all = False

        if divides_all:
            break
        else:
            print(n, "nope")
            n += 1

    return n
