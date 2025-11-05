#include <iostream>

#include "field_computer_external_api_idl/connection/Hello.hpp"
#include "field_computer_external_api_idl/connection/constants.hpp"

int main() {
    std::cout << "Schema version: " << connection::schema_major << "." << connection::schema_minor << "." << connection::schema_patch << std::endl;
    return 0;
}
