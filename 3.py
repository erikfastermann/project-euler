import sys

n = int(sys.argv[1])
biggest_prime = None

while n%2 == 0:
    biggest_prime = 2
    n //= 2

i = 3
while n > 2:
    while n%i == 0:
        biggest_prime = i
        n //= i
    i += 2

print(biggest_prime)