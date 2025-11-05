#include <iostream>

#include "field_computer_external_api_idl/connection/Hello.hpp"
#include "field_computer_external_api_idl/connection/constants.hpp"

int main() {
    std::cout << "Schema version: " << connection::SCHEMA_MAJOR_VERSION << "." << connection::SCHEMA_MINOR_VERSION << "." << connection::SCHEMA_PATCH_VERSION << std::endl;
    return 0;
}
