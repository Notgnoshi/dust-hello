add_executable(hello ./main.cpp)
target_compile_features(hello PRIVATE cxx_std_17)

find_package(Hurrier REQUIRED)
target_link_libraries(hello PRIVATE hurrier::hurrier)

find_package(FieldComputerExternalApiIdl REQUIRED)
target_link_libraries(hello PRIVATE field_computer_external_api_idl)
