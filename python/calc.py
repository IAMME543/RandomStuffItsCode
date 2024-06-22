operation = input("Operation: ")
numA = float(input("Num A: "))
numB = float( input("Num B: "))

if operation == "*":
    print(f"{numA} {operation} {numB} = {numA * numB}")
elif operation == "+":
    print(f"{numA} {operation} {numB} = {numA + numB}")
elif operation == "/":
    print(f"{numA} {operation} {numB} = {numA / numB}")
elif operation == "-":
    print(f"{numA} {operation} {numB} = {numA - numB}")
else:
    print("Was that a valid operation?")