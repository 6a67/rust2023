#include <iostream>
#include <variant>

struct Error
{
    std::string message;
};

struct Warning
{
    std::string message;
    int code;
};

using Status = std::variant<Error, Warning>;

void printStatus(const Status &status)
{
    // check for variant and act accordingly
    if (auto value = std::get_if<Error>(&status))
    {
        std::cout << "Error: " << value->message << std::endl;
    }
    else if (auto value = std::get_if<Warning>(&status))
    {
        std::cout << "Warning: " << value->message
                  << " (" << value->code << ")" << std::endl;
    }
}

int main()
{
    Status status = Error{"Error while reading file"};
    printStatus(status);
    status = Warning{"File not found", 404};
    printStatus(status);
    return 0;
}