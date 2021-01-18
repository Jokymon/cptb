project({{projectname}})

add_executable(${PROJECT_NAME})

target_sources(${PROJECT_NAME}
    PRIVATE
        src/main.cpp
)

include(cmake/CPM.cmake)

{{#if static_build}}
target_link_libraries({{projectname}} -static-libgcc -static-libstdc++)
{{/if}}
