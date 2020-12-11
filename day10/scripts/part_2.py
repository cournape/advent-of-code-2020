import collections
import sys

with open(sys.argv[1]) as fp:
    values = {int(entry) for entry in fp}

values.add(0)

g = collections.defaultdict(list)
for parent in values:
    for candidate in range(parent + 1, parent + 4):
        if candidate in values:
            g[parent].append(candidate)

low, end = min(values), max(values)

visited = {}

# We're assuming there is no cycle
def visit(parent, term):
    if parent == term:
        return 1
    else:
        if not parent in visited:
            visited[parent] = sum(visit(child, term) for child in g[parent])
        return visited[parent]

print(f"Found {visit(low, end)} paths")
