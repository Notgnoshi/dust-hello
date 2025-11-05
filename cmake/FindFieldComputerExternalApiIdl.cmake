if(NOT FieldComputerExternalApiIdl_FOUND)
    add_subdirectory(
        "${PROJECT_SOURCE_DIR}/submodules/field-computer-external-api-idl"
        "${CMAKE_BINARY_DIR}/field-computer-external-api-idl"
    )
    set(FieldComputerExternalApiIdl_FOUND TRUE)
endif()
