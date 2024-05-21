from functools import reduce


def solve_one():
    return reduce(
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


def solve_six():
    sum_of_sqs = reduce(lambda x, y: x + y, [pow(x, 2) for x in range(1, 101)])
    sq_of_sums = pow(reduce(lambda x, y: x + y, [x for x in range(1, 101)]), 2)

    return sq_of_sums - sum_of_sqs


def solve_eight():
    input = """73167176531330624919225119674426574742355349194934
            96983520312774506326239578318016984801869478851843
            85861560789112949495459501737958331952853208805511
            12540698747158523863050715693290963295227443043557
            66896648950445244523161731856403098711121722383113
            62229893423380308135336276614282806444486645238749
            30358907296290491560440772390713810515859307960866
            70172427121883998797908792274921901699720888093776
            65727333001053367881220235421809751254540594752243
            52584907711670556013604839586446706324415722155397
            53697817977846174064955149290862569321978468622482
            83972241375657056057490261407972968652414535100474
            82166370484403199890008895243450658541227588666881
            16427171479924442928230863465674813919123162824586
            17866458359124566529476545682848912883142607690042
            24219022671055626321111109370544217506941658960408
            07198403850962455444362981230987879927244284909188
            84580156166097919133875499200524063689912560717606
            05886116467109405077541002256983155200055935729725
            71636269561882670428252483600823257530420752963450"""
    n = list(filter(lambda c: c != "\n" and c != " ", [*input]))
    i = 0
    j = 12

    output = 0
    while j <= len(n):
        output = max(output, reduce(
            lambda x, y: x*y, [int(x) for x in n[i:j+1]]))

        i += 1
        j += 1

    return output


def solve_nine():
    def prop_one(a, b, c): return a < b and b < c
    def prop_two(a, b, c): return pow(a, 2) + pow(b, 2) == pow(c, 2)

    for i in range(1000):
        for j in range(i, 1000):
            for k in range(j, 1000):
                if i+j+k == 1000 and prop_one(i, j, k) and prop_two(i, j, k):
                    return i*j*k


def solve_eleven():
    input = """
        08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
        49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
        81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
        52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
        22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
        24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
        32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
        67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
        24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
        21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
        78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
        16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
        86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
        19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
        04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
        88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
        04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
        20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
        20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
        01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
    """

    rows = input.strip().split('\n')
    grid = [[int(x) for x in row.split()] for row in rows]

    # n->s
    max_prod_col = 0
    for i in range(17):
        for j in range(20):
            prod = grid[i][j]
            prod *= grid[i+1][j]
            prod *= grid[i+2][j]
            prod *= grid[i+3][j]
            max_prod_col = max(max_prod_col, prod)

    # w->e
    max_prod_row = 0
    for row in grid:
        i = 0
        j = 4

        while j <= len(row):
            prod = reduce(lambda x, y: x*y, row[i:j])
            max_prod_row = max(max_prod_row, prod)

            i += 1
            j += 1

    # nw->se
    max_prod_diag_se = 0
    for i in range(17):
        for j in range(17):
            prod = grid[i][j]
            prod *= grid[i+1][j+1]
            prod *= grid[i+2][j+2]
            prod *= grid[i+3][j+3]
            max_prod_diag_se = max(max_prod_diag_se, prod)

    # ne->sw
    max_prod_diag_sw = 0
    for i in range(17):
        for j in range(3, 20):
            prod = grid[i][j]
            prod *= grid[i+1][j-1]
            prod *= grid[i+2][j-2]
            prod *= grid[i+3][j-3]
            max_prod_diag_sw = max(max_prod_diag_sw, prod)

    return max(max_prod_row, max_prod_col, max_prod_diag_se, max_prod_diag_sw)
