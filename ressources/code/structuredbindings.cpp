#include <iostream>
#include <tuple>

int main()
{
    std::tuple<int, std::string> tup{42, "hello"};

    auto [i, str] = tup;

    std::cout << "i: " << i << std::endl;
    std::cout << "str: " << str << std::endl;

    return 0;
}
