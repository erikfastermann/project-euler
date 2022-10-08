with open('p089_roman.txt') as f:
    roman = [line.strip() for line in f]

numerals = [
    ('M', 1000),
    ('D', 500),
    ('C', 100),
    ('L', 50),
    ('X', 10),
    ('V', 5),
    ('I', 1),
]

numerals_lookup = {n[0]:n[1] for n in numerals}

numerals_sub = [
    ('CM', 900), 
    ('CD', 400), 
    ('XC', 90),  
    ('XL', 40),  
    ('IX', 9),   
    ('IV', 4),   
]

numerals_sub_lookup = {s[0]:s[1] for s in numerals_sub}

def roman_to_int(r):
    total = 0
    i = 0
    while i < len(r):
        if i+1 < len(r) and r[i:i+2] in numerals_sub_lookup:
            total += numerals_sub_lookup[r[i:i+2]]
            i += 2
        else:
            total += numerals_lookup[r[i]]
            i += 1
    return total

def int_to_roman(n):
    r = ''
    for i, numeral in enumerate(numerals):
        numeral_count = (n // numeral[1])
        r += numeral[0] * numeral_count
        n -= numeral_count * numeral[1]

        if i < len(numerals_sub):
            potential_sub = n // numerals_sub[i][1]
            assert 0 <= potential_sub <= 1
            if potential_sub == 1:
                r += numerals_sub[i][0]
                n -= numerals_sub[i][1]
    return r

savings = 0
for r in roman:
    optimal = int_to_roman(roman_to_int(r))
    savings += len(r) - len(optimal)

print(savings)
