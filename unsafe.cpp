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

    try
    {
        a.at(3) = 0x7ffff7b36cebUL;
        std::cout << a.at(3) << '\n';
    }
    catch (std::out_of_range &error)
    {
        std::cout << "Failed to set requested index: " << error.what() << '\n';
    }
    return 0;
}
