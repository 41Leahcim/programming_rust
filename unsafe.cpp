#include <stdio.h>

int main(const int argc, const char **argv)
{
    for (int i = 0; i < argc; i++)
    {
        printf("\"%s\" ", argv[i]);
    }
    putchar('\n');
    unsigned long a[1];

    // C allows this, but behaviour is unpredictable
    a[3] = 0x7ffff7b36cebUL;
    printf("%lu\n", a[3]);
    return 0;
}
