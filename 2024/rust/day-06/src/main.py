
txt_test = """
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"""

def process_one(txt):
    # Read input from file

    lines = []
    for line in txt.strip().split('\n'):
        lines.append(list(line))

    m = len(lines)
    n = len(lines[0])
    x, y = 0, 0

    for i in range(m):
        for j in range(n):
            if lines[i][j] == '^':
                x, y = i, j
                break

    vs = set([(x, y)])
    direction = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    i = 0
    while 0 <= x < m and 0 <= y < n:
        nx, ny = x + direction[i][0], y + direction[i][1]
        if nx < 0 or nx >= m or ny < 0 or ny >= n:
            break
        if lines[nx][ny] == '#':
            i += 1
            i %= 4
        else:
            vs.add((nx, ny))

            x, y = nx, ny

    return vs
    # return len(vs)
    # part2
    ## we have x, y, direction: if this is visted, we can be sure that
    ## there's a loop

def process_two(txt):
    # Read input from file

    lines = []
    for line in txt.strip().split('\n'):
        lines.append(list(line))

    m = len(lines)
    n = len(lines[0])

    x, y = 0, 0
    for i in range(m):
        for j in range(n):
            if lines[i][j] == '^':
                x, y = i, j
                break

    startx, starty = x, y
    all_node = process_one(txt)
    all_node.remove((x, y))

    res = 0
    for p, q  in all_node:
        # if p != 9 or q!= 7:
        #     continue
        lines[p][q] = "#"
        loop = False
        x, y = startx, starty

        vs = set([(x, y, 0)])
        direction = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        i = 0
        while 0 <= x < m and 0 <= y < n:
            nx, ny = x + direction[i][0], y + direction[i][1]
            if nx < 0 or nx >= m or ny < 0 or ny >= n:
                break
            if lines[nx][ny] == '#':
                i += 1
                i %= 4
            else:
                if (nx, ny, i) in vs:
                    res +=1
                    break

                vs.add((nx, ny, i))
                x, y = nx, ny

        lines[p][q] = "."

    return res

import os
# Get the directory containing the script
script_dir = os.path.dirname(os.path.abspath(__file__))
# Go up one level to the day-05 directory
input_path = os.path.join(script_dir, "..", "input2.txt")

if __name__ == "__main__":
    # with open(input_path) as f:
    #     txt = f.read()
    #     pairs, lists = parse_input(txt)
    #     print(process(pairs, lists))

    with open(input_path) as f:
        txt = f.read()
        print(process_two(txt))
