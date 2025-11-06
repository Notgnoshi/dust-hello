# First look for it in the environment provided by toolchain
if(NOT TARGET cnhos-log::cnhos-log)
    find_package(cnhos-log CONFIG QUIET)

    if(cnhos-log_FOUND)
        message(STATUS "Found cnhos-log @ ${cnhos-log_DIR}")
    endif()
endif()

if(NOT TARGET cnhos-log::cnhos-log)
    include(FetchContent)

    FetchContent_Declare(
        cnhos-log
        GIT_REPOSITORY git@gitlab.cnh.com:cnhitech/middleware/cnhos-log-lib.git
        GIT_TAG 7e75ba948472fb3e20f7da3936e13af97fd1e796
    )

    set(CNHOSLOG_BUILD_STDOUT_SHARED ON)
    set(CNHOSLOG_BUILD_LOG4CPLUS_SHARED OFF)

    message(STATUS "Syncing cnhos-log repo...")
    FetchContent_MakeAvailable(cnhos-log)

    set(cnhos-log_FOUND TRUE)
endif()
