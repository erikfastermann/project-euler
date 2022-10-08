from fractions import Fraction

def is_bouncy(n):
    digits = str(n)
    last = int(digits[0])
    asc = False
    desc = False
    for digit in digits[1:]:
        digit = int(digit)
        if digit > last:
            asc = True
        elif digit < last:
            desc = True
        last = digit
    return asc and desc

required_proportion = Fraction(99, 100)

n = 1
bouncy = 0
while True:
    if is_bouncy(n):
        bouncy += 1

    proportion = Fraction(bouncy, n)
    if proportion == required_proportion:
        break

    n += 1

print(n)