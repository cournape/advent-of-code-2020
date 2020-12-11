import sys

counts = {"1": 0, "3": 1}
for line in iter(sys.stdin.readline, b''):
    if len(line) < 1:
        break
    counts[line.strip()] += 1

print(counts)
