from math import sqrt

# Any factors of n must be less than sqrt(n):
#     a*b > sqrt(n)*sqrt(n) => a*b % n != 0
# ==> factors (a,b) < sqrt(n)


def prime(n):
    if n == 0 or n == 1:
        return False

    for i in range(2, sqrt(n)+1):
        if n % i == 0:
            return False

    return True


def factors(n):
    if n == 0 or n == 1:
        return []

    output = []
    for i in range(2, sqrt(n)+1):
        while n % i == 0:
            output.append(i)
            n /= i

    output.append(n)

    return output


def sieve(n):
    return 0


def num_of_factors(n):
    return 0


def sum_of_factors(n):
    return 0


def prod_of_factors(n):
    return 0


def gcd(a, b):
    return 0


def lcm(a, b):
    return 0


def eea(a, b):
    return 0


def crt(a, b):
    return 0
