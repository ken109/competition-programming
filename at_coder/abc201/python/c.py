from itertools import combinations_with_replacement, permutations

s = input()

must = []
ok = []

for i, c in enumerate(s):
    if c == 'o':
        must.append(i)
    if c != 'x':
        ok.append(i)

if len(must) > 4:
    print(0)
    exit(0)

ans_set = set()

for remain in combinations_with_replacement(ok, 4 - len(must)):
    combination = must[:]
    combination.extend(remain)
    for permutation in permutations(combination, 4):
        ans_set.add(permutation)

print(len(ans_set))
