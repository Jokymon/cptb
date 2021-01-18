cmake_minimum_required(VERSION 3.0)
PROJECT(get_settings)

if (MINGW)
    set(TOOLCHAIN "MinGW")
elseif(GCC)
    set(TOOLCHAIN "GCC")
elseif(MSVC)
    set(TOOLCHAIN "MSVC")
else()
    set(TOOLCHAIN "Other")
endif()

if (WIN32)
    set(TARGET "WIN32")
elseif(UNIX)
    set(TARGET "UNIX")
else()
    set(TARGET "Other")
endif()

file(CONFIGURE OUTPUT status.json
    CONTENT
    "{ 
       \"toolchain\": \"${TOOLCHAIN}\",
       \"target\": \"${TARGET}\",
       \"cmake_version\": \"${CMAKE_VERSION}\"
    }"
)

add_executable(main main.cpp)