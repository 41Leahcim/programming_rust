#include <iostream>
#include <array>

int main(const int argc, const char **argv)
{
    for (int i = 0; i < argc; i++)
    {
        std::cout << '"' << argv[i] << "\" ";
    }
    std::cout << '\n';
    std::array<unsigned long, 1> a;

    // C allows this, but behaviour is unpredictable
    a[3] = 0x7ffff7b36cebUL;
    printf("%lu\n", a[3]);
    return 0;
}
