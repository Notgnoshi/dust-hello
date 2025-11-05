# if(NOT FieldComputerExternalApiIdl_FOUND)
#     find_package(FieldComputerExternalApiIdl CONFIG QUIET)
#     message(STATUS "Searching for FieldComputerExternalApiIdl in toolchain")
#
#     if(FieldComputerExternalApiIdl_FOUND)
#         message(STATUS "Found FieldComputerExternalApiIdl (${FieldComputerExternalApiIdl_DIR})")
#     else()
#         message(STATUS "FieldComputerExternalApiIdl not found in toolchain")
#     endif()
# endif()

if(NOT FieldComputerExternalApiIdl_FOUND)
    include(FetchContent)

    FetchContent_Declare(
        FieldComputerExternalApiIdl
        GIT_REPOSITORY git@gitlab.cnh.com:atd/idl/field-computer-external-api-idl.git
        GIT_TAG 82e69a36b51683dd70c640d9e628c7a99e796d2b
    )

    message(STATUS "Syncing FieldComputerExternalApiIdl repo...")
    FetchContent_MakeAvailable(FieldComputerExternalApiIdl)
    set(FieldComputerExternalApiIdl_FOUND TRUE)
endif()
