ans = 0

numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

with open("in", "r") as f:
    for line in f:
        nums = []
        line = line.strip()
        for k in range(len(line)):
            if line[k] in "0123456789":
                nums.append(line[k])
                continue
            for i, num in enumerate(numbers):
                if k + len(num) - 1 < len(line):
                    if line[k:k+len(num)] == num:
                        nums.append(str(i+1))

        ans +=  int(nums[0]) * 10 + int(nums[-1])

print(ans)