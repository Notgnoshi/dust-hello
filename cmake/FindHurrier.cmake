if(NOT Hurrier_FOUND)
    find_package(Hurrier CONFIG QUIET)
    message(STATUS "Searching for Hurrier in toolchain")

    if(Hurrier_FOUND)
        message(STATUS "Found Hurrier (${Hurrier_DIR})")
    else()
        message(STATUS "Hurrier not found in toolchain")
    endif()
endif()

if(NOT Hurrier_FOUND)

    include(FetchContent)

    FetchContent_Declare(
        hurrier
        GIT_REPOSITORY git@gitlab.cnh.com:cnhitech/middleware/hurrier-ipc/hurrier.git
        GIT_TAG 6aabcc42c1044023799818f17120a418d62b2fd9
    )

    set(HURRIER_BUILD_FASTCDR ON)
    set(HURRIER_BUILD_FASTRTPS ON)
    set(HURRIER_BUILD_FOONATHAN ON)
    set(HURRIER_BUILD_GTEST OFF)
    set(HURRIER_BUILD_YAML ON)
    set(HURRIER_BUILD_CYCLONEDDS OFF)

    set(HURRIER_FETCHCONTENT_DISABLE OFF)
    set(HURRIER_ENABLE_UNIT_TESTS OFF)
    set(HURRIER_EXAMPLES OFF)

    message(STATUS "Syncing hurrier repo...")
    FetchContent_MakeAvailable(hurrier)

    set(Hurrier_FOUND TRUE)
endif()
