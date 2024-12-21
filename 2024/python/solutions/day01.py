import collections


def solve():
    filename = "inputs/day01.input"
    with open(filename) as f:
        content = f.readlines()

    left_list = []
    right_list = []
    for line in content:
        left,right = line.split('   ', 1)
        left_list.append(int(left.strip()))
        right_list.append(int(right.strip()))

    left_list.sort()
    right_list.sort()
    sum = 0
    for i in range(0, len(left_list)):
        sum += abs(left_list[i] - right_list[i])
    print(sum)



def bonus():
    filename = "inputs/day01.input"
    with open(filename) as f:
        content = f.readlines()

    left_list = []
    right_list = []
    for line in content:
        left,right = line.split('   ', 1)
        left_list.append(int(left.strip()))
        right_list.append(int(right.strip()))

    left_list.sort()
    right_counter = collections.Counter(right_list)

    sum = 0

    for num in left_list:
        if (right_counter.get(num) is None):
            # sum += 0
            continue
        sum += (num * right_counter.get(num))

    print(str.format("Sum: {}", sum))
