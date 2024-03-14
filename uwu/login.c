#include <stdio.h>
#include <string.h>

int main()
{
    char expected[10] = "secret";
    char user[5];

    printf("username:");
    gets(user); // vulnerable function, doesn't check buffer size
    // scanf("%s", &user);
    printf("password:");
    char password[100];
    gets(password);

    if (strcmp(expected, password) == 0)
    {
        printf("uwu: authenticated!!\n");
    }
    else
    {
        printf("uwu: wrong password for the '%s' expected a different secret\n", user);
    }
    return 0;
}