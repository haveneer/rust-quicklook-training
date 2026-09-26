cmake_minimum_required(VERSION 3.22)
project(cxx_cmake_interface LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Corrosion: CMake integration of cargo (https://github.com/corrosion-rs/corrosion)
include(FetchContent)
FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.6.1
)
FetchContent_MakeAvailable(Corrosion)

# Existing C++ code, unaware of Rust
add_library(field STATIC cpp/field.cpp)
target_include_directories(field PUBLIC cpp)

# Rust crate -> CMake target `sim_rs` (staticlib, built with cargo under the hood;
# its build.rs runs autocxx on cpp/field.hpp)
corrosion_import_crate(MANIFEST_PATH Cargo.toml CRATES cxx-cmake-interface)
if (APPLE)
    # Corrosion hard-codes -L<CommandLineTools SDK>/usr/lib, which breaks the link when
    # that SDK does not match the active toolchain (e.g. Xcode); the compiler's SDK is enough.
    set_property(TARGET sim_rs-static PROPERTY INTERFACE_LINK_DIRECTORIES "")
endif ()

# #[cxx::bridge] in src/bridge.rs (Rust -> C++) -> C++ library `sim_rs_bridge`
# (generated header: sim_rs_bridge/bridge.h, plus rust/cxx.h)
# cxxbridge-cmd is installed automatically, matching the `cxx` version in Cargo.lock
corrosion_add_cxxbridge(sim_rs_bridge CRATE sim_rs FILES bridge.rs)
target_link_libraries(sim_rs_bridge PUBLIC field)

add_executable(demo cpp/main.cpp)
target_link_libraries(demo PRIVATE sim_rs_bridge sim_rs field)
