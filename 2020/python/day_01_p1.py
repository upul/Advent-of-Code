def find_two(target, numbers):
    for number in numbers:
        if target - number in numbers:
            return number, (target - number)
    return ()

def find_three(target, numbers):
    for number in numbers:
        next_two = target - number
        res = find_two(next_two, numbers)
        if res:
            return (number, res[0], res[1])
    return ()

with open("../data/day_01_p1.txt") as fp:
    data = fp.read()

expenses = [int(x) for x in data.split()]
unique_expenses = set(expenses)
print(f"size of the expense report: {len(expenses)}")
print(f"number of unique expenses: {len(unique_expenses)}")

for curr_expense in expenses:
    remaining_expense = 2020 - curr_expense
    if remaining_expense in unique_expenses:
        expense_product = curr_expense * remaining_expense
        print(f"two expenses: {curr_expense}, {remaining_expense} and product: {expense_product}")
        break 

x, y = find_two(2020, expenses)
print(f"{x}, {y}, {x + y}, {x*y}")


x, y, z = find_three(2020, expenses)
print(f"{x}, {y}, {z}, {x+y+z}, {x*y*z}")




