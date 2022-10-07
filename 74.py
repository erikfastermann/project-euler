import math

def digit_factorial_sum(n):
    return sum(math.factorial(int(i)) for i in str(n))

with_60 = 0
for i in range(1_000_000):
    current = i
    found = set()
    while current not in found:
        found.add(current)
        current = digit_factorial_sum(current)
    if len(found) == 60:
        with_60 += 1

print(with_60)