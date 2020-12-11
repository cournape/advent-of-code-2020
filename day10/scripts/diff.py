import sys

previous = 0
for line in iter(sys.stdin.readline, b''):
    if len(line) < 1:
        break
    print(int(line) - previous)
    previous = int(line)
