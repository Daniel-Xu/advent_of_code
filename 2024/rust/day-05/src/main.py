from collections import defaultdict

txt_test = """
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"""

def parse_input(txt):
    # Split into two parts based on double newline
    parts = txt.strip().split("\n\n")

    # Parse first part - pairs of numbers
    pairs = []
    for line in parts[0].split("\n"):
        if line:  # Skip empty lines
            a, b = line.split("|")
            pairs.append((int(a), int(b)))

    # Parse second part - lists of numbers
    lists = []
    for line in parts[1].split("\n"):
        if line:  # Skip empty lines
            numbers = [int(x) for x in line.split(",")]
            lists.append(numbers)

    return pairs, lists

def process(rules, orders):
    # rules: 47 -> pages needs to after 47

    vs = defaultdict(set)
    for a, b in rules:
        vs[a].add(b)

    # 75,47,61,53,29
    res = 0
    for order in orders:
        safe = True
        for i in range(len(order)):
            for j in range(i + 1, len(order)):
                if order[i] in vs[order[j]]:
                    safe = False
                    print("unsafe", order)
                    break
            if not safe:
                break

        if safe:
            res += order[len(order) // 2]

    return res

def unsafe_orders(vs, orders):
    unsafe = []
    for order in orders:
        safe = True
        for i in range(len(order)):
            for j in range(i + 1, len(order)):
                if order[i] in vs[order[j]]:
                    safe = False
                    break
            if not safe:
                unsafe.append(order[:])
                break

    return unsafe

def process2(vs, orders):
    res = 0
    for order in orders:
        i = 0
        while i < len(order):
            switched = False
            for j in range(i+1, len(order)):
                if order[i] in vs[order[j]]:
                    order[i], order[j] = order[j], order[i]
                    switched = True
                    break
            if not switched:
                i += 1
        res += order[len(order) // 2]
    print(res)

import os

# Get the directory containing the script
script_dir = os.path.dirname(os.path.abspath(__file__))
# Go up one level to the day-05 directory
input_path = os.path.join(script_dir, "..", "input1.txt")

if __name__ == "__main__":
    # with open(input_path) as f:
    #     txt = f.read()
    #     pairs, lists = parse_input(txt)
    #     print(process(pairs, lists))

    with open(input_path) as f:
        txt = f.read()
        pairs, lists = parse_input(txt)
        vs = defaultdict(set)
        for a, b in pairs:
            vs[a].add(b)

        unsafe = unsafe_orders(vs, lists)
        process2(vs, unsafe)
