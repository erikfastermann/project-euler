import itertools
from fractions import Fraction

def e_fraction():
    yield Fraction(1, 1)
    i = Fraction(2, 1)
    while True:
        yield i
        yield 1
        yield 1
        i += 2

def convergent(nth, fraction):
    data = list(itertools.islice(fraction, nth-1))
    res = Fraction(0, 1)
    for i in reversed(data):
        res = 1 / (i + res)
    return res

x = 2 + convergent(100, e_fraction())
print(sum(int(i) for i in str(x.numerator)))