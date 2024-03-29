cmake_minimum_required(VERSION 3.12)
project(rust-quicklook)
option(ENABLE_STATIC_ANALYSIS "Enable static analysis" OFF)
set(CPP_TRAINING_SOURCE_DIR ${PROJECT_SOURCE_DIR}) # to locate main root source in sub-projects

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
add_definitions(-DDEBUG_TYPE)
add_definitions(-DSAFE_RUN) # add workaround for explicitly buggy example (or on the edge behavior)

#------------------------------------------------------

if (ENABLE_STATIC_ANALYSIS)
    # search for clang-tidy (while be used while adding library or executable)
    find_program(CLANG_TIDY clang-tidy)
    if (CLANG_TIDY)
        set(CMAKE_CXX_CLANG_TIDY ${CLANG_TIDY})
        message(STATUS "Static analysis enabled while compiling.")
    else ()
        message(FATAL_ERROR "Static analysis requested but not available.")
    endif ()
endif ()

#------------------------------------------------------

if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -Wall -Wextra -pedantic")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-unused -Wno-unused-parameter")
    # To focus only on useful warnings 
    # set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}  -Wunused-function -Wno-unused-variable -Wno-unused-parameter -Wno-unused-value -Wno-unused-result -Wno-unused-but-set-variable")
elseif (CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "AppleClang")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -stdlib=libc++")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -Wall -Wextra -pedantic")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-unused -Wno-unused-parameter")
    # To focus only on useful warnings 
    # set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wunused-member-function -Wunused-private-field -Wno-unused-variable -Wno-unused-parameter -Wno-unused-value -Wno-unused-result -Wno-unused-but-set-variable")
elseif (CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /W4 /Zc:__cplusplus")
elseif (CMAKE_CXX_COMPILER_ID MATCHES "PGI")
    set(PGI TRUE)
    set(CMAKE_CXX_STANDARD 17) # C++20 requires an updated compiler (not yet available) 
endif ()

#------------------------------------------------------

if (NOT WIN32)
    set(THREADS_PREFER_PTHREAD_FLAG ON)
    find_package(Threads REQUIRED)
    link_libraries(Threads::Threads)
endif ()

#------------------------------------------------------
# Check compiler support (when __cpp_* feature is not available)
include(CheckCXXSourceCompiles)

# https://en.cppreference.com/w/cpp/memory/shared_ptr/make_shared (in 4-Notes)
check_cxx_source_compiles("#include <memory>\n int main() { auto a = std::make_shared<int[10]>(); }"
        HAS_CXX_FEATURE_SHARED_PTR_ARRAYS)
if (HAS_CXX_FEATURE_SHARED_PTR_ARRAYS)
    ADD_COMPILE_DEFINITIONS(my_cpp_lib_shared_ptr_arrays)
endif ()

#------------------------------------------------------

add_executable(manage_test_crash tools/manage_test_crash.c)

message(STATUS "Detect file to compile using globbing")
file(GLOB SRCS1 RELATIVE "${CMAKE_SOURCE_DIR}" "src/*.cpp")
file(GLOB SRCS2 RELATIVE "${CMAKE_SOURCE_DIR}" "extras/*.cpp")
set(SRCS ${SRCS1} ${SRCS2})

include_directories(include)

#------------------------------------------------------

set(EXCLUDED)
if (MSVC)
    list(APPEND EXCLUDED)
    if (NOT DONT_VS_EXCLUDE)
        list(APPEND EXCLUDED)
    endif ()
endif ()

if (PGI)
    # compile time errors
    list(APPEND EXCLUDED)
    # runtime errors
    list(APPEND EXCLUDED)
endif ()

#------------------------------------------------------

enable_testing()
foreach (SRC ${SRCS})
    get_filename_component(SHORT_SRC ${SRC} NAME)
    string(REGEX REPLACE ".cpp$" ".exe" EXE ${SHORT_SRC})
    list(FIND EXCLUDED ${SRC} IS_EXCLUDED)
    if (IS_EXCLUDED GREATER_EQUAL 0)
        message(STATUS "\tExclude example ${SRC}")
    else ()
        set(SRCS ${SRC})

        set(TYPE "example")
        string(REGEX MATCH "--.*\\.cpp$" SKIP_COMPANION ${SRC})
        if (SKIP_COMPANION)
            continue()
        else ()
            string(REGEX REPLACE ".cpp$" "" TEST ${SRC})
        endif ()
        file(GLOB EXTRAS_PATH RELATIVE "${CMAKE_SOURCE_DIR}" "${TEST}--*.*")

        set(EXTRAS)
        foreach (S ${EXTRAS_PATH} ${GTEST_EXTRAS_PATH})
            if (NOT S IN_LIST SRCS)
                get_filename_component(EXTRA ${S} NAME)
                list(APPEND SRCS ${S})
                list(APPEND EXTRAS ${EXTRA})
            endif ()
        endforeach ()

        if (EXTRAS)
            message(STATUS "\tAdd ${TYPE} ${SRC} with extras: ${EXTRAS}")
        else ()
            message(STATUS "\tAdd ${TYPE} ${SRC}")
        endif ()

        add_executable(${EXE} ${SRCS})

        set(WILL_FAIL_FOUND 0)
        string(FIND ${SRC} "-will_fail" WILL_FAIL_FOUND)
        if (${WILL_FAIL_FOUND} EQUAL "-1")
            add_test(NAME ${TEST} COMMAND ${EXE})
        else ()
            add_test(NAME ${TEST} COMMAND manage_test_crash ${EXE})
            set_tests_properties(${TEST} PROPERTIES WILL_FAIL TRUE)
        endif ()
    endif ()
endforeach ()
