# coding: utf-8
with open('input.txt') as f:
    expenses = [int(line) for line in f.readlines()]
    
# part 1
pairs = [
        (a, b)
        for a in expenses for b in expenses
        if a != b and a + b == 2020
        ]
a , b = pairs[0]
print(a * b)

# part 2
triples = [
        (a, b, c)
        for a in expenses for b in expenses for c in expenses
        if a != b and b != c and a != c and a + b + c == 2020
        ]
a, b, c = triples[0]
print(a * b * c)
