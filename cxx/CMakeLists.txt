add_executable(hello ./main.cpp)
target_compile_features(hello PRIVATE cxx_std_17)

find_package(Hurrier REQUIRED)
target_link_libraries(hello PRIVATE hurrier::hurrier)
