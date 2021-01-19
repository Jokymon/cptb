cmake_minimum_required(VERSION {{cmake_version}})
include(cmake/CPM.cmake)

{{#if with_tests}}
CPMAddPackage(
    NAME Catch2
    GITHUB_REPOSITORY catchorg/Catch2
    VERSION 2.5.0
)
{{/if}}

project({{projectname}})

add_executable(${PROJECT_NAME})
target_sources(${PROJECT_NAME}
    PRIVATE
        src/main.cpp
)
{{#if with_tests}}
add_executable(tests)
target_sources(tests
    PRIVATE
        tests/main.cpp
)
target_link_libraries(tests
    PRIVATE
        Catch2
)
{{/if}}
{{#if static_build}}
{{#if (eq toolchain "MinGW")}}
target_link_libraries({{projectname}} -static-libgcc -static-libstdc++)
{{/if}}
{{/if}}
