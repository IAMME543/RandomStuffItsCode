package main

import "fmt"

func main() {

	//variables
	var operation string
	var numA float32
	var numB float32

	fmt.Println("Operation: ")
	fmt.Scanln(&operation)
	fmt.Println("Number A: ")
	fmt.Scanln(&numA)
	fmt.Println("Number B: ")
	fmt.Scanln(&numB)

	switch operation {
	case "+":
		fmt.Println(numA, operation, numB, "=", numA+numB)
	case "*":
		fmt.Println(numA, operation, numB, "=", numA*numB)
	case "/":
		fmt.Println(numA, operation, numB, "=", numA/numB)
	case "-":
		fmt.Println(numA, operation, numB, "=", numA+numB)
	default:
		fmt.Println("Was that a valid operation?")
		break
	}
	fmt.Scanln()

}
