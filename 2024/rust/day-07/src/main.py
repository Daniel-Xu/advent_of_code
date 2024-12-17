
txt_test = """
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"""



def handle_input(txt):

# return structure of
# reuslt: u32
# element: list of numbers
    equations = []
    for line in txt.strip().split('\n'):
        if not line:
            continue
        result, elements = line.split(': ')
        result = int(result)
        elements = [int(x) for x in elements.split()]
        equations.append({
            'result': result,
            'elements': elements
        })
    return equations


def traverse(elements, path, result, depth, expected):
    #     +         *
    # +     *    +   *
    if result > expected:
        return None

    if len(path) == len(elements) - 1:
        return result

    for operator in ['+', '*', "||"]:
        path.append(operator)
        if operator == '+':
            result += elements[depth]
        elif operator == '*':
            result *= elements[depth]
        elif operator == "||":
            result = int(str(result) + str(elements[depth]))
        v = traverse(elements, path, result, depth + 1, expected)
        if v == expected:
            return v
        path.pop()
        if operator == '+':
            result -= elements[depth]
        elif operator == '*':
            result //= elements[depth]
        elif operator == "||":
            result = int(str(result)[:-len(str(elements[depth]))])

def process(txt):
    equations = handle_input(txt)
    res = 0
    for equation in equations:
        elements = equation['elements']
        expected = equation['result']
        if traverse(elements, [], elements[0], 1, expected):
            res += expected
    print(res)

import os
# Get the directory containing the script
script_dir = os.path.dirname(os.path.abspath(__file__))
# Go up one level to the day-05 directory
input_path = os.path.join(script_dir, "..", "input2.txt")

# python main
if __name__ == "__main__":
    with open(input_path) as f:
        txt = f.read()
        process(txt)

    # process(txt_test)
