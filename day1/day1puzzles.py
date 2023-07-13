###!/bin/env python
with open("elfCals.txt", 'r') as file:
    cals = 0
    cal_list = []
    for line in file.readlines():
        num = line.strip()
        if num.isdigit():
            cals += int(num)
        else:
            cal_list.append(cals)
            cals = 0
    cal_list.sort(reverse=True)
    print(cal_list[0] + cal_list[1] + cal_list[2])
    print(max(cal_list))
