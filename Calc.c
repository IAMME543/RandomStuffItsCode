#include <stdio.h>

int main()
{

    while (1)
    {
        // Variables
        char operation;
        float a;
        float b;

        printf("Operation: ");
        scanf(" %c", &operation);

        printf("A: ");
        scanf("%f", &a);

        printf("B: ");
        scanf("%f", &b);

        switch (operation) {
        case '*':
            printf("%f\n", a * b);
            break;

        case '/':
            printf("%f\n", a / b);
            break;

        case '+':
            printf("%f\n", a + b);
            break;

        case '-':
            printf("%f\n", a - b);
            break;

        default:
            printf("Was That a valid operation?\n");
            break;
        }
    }
}