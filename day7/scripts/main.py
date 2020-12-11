import collections
import pprint
import re

BAG_R = re.compile("(\d+) (\w+ \w+) bag")
MY_BAG = "shiny gold"

def parse_entry(line):
    left, right = [s.strip() for s in line.split("bags contain")]
    return left, {m.group(2): int(m.group(1)) for m in BAG_R.finditer(right)}

with open("input") as fp:
    # parent to children
    p_to_c = collections.defaultdict(collections.Counter)
    for line in fp:
        parent, children = parse_entry(line)
        p_to_c[parent].update(collections.Counter(children))

    # child to parents
    c_to_p = collections.defaultdict(list)
    for parent, children in p_to_c.items():
        for child in children:
            c_to_p[child].append(parent)

    ancestors = set()

    # DFS
    def visit(ancestor):
        ancestors.add(ancestor)
        parents = c_to_p[ancestor]
        for parent in parents:
            visit(parent)

    for ancestor in c_to_p[MY_BAG]:
        visit(ancestor)

    print(len(ancestors))

    # Convert to "normal" dict for pretty printing for debug
    p_to_c = {k: dict(v) for k, v in p_to_c.items()}

    def visit(parent):
        children = p_to_c[parent]
        count = 0
        for child in children:
            count += children[child] * (1 + visit(child))
        return count

    print(visit(MY_BAG))
