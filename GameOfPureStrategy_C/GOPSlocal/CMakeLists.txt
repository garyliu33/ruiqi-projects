cmake_minimum_required(VERSION 3.10)
project(GOPSlocal C)

set(CMAKE_C_STANDARD 99)
set(SOURCE_FILES src/main.c src/card.c src/deck.c src/player.c src/prize.c)

add_executable(GOPSlocal ${SOURCE_FILES})

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK REQUIRED gtk4)

include_directories(${GTK_INCLUDE_DIRS})
link_directories(${GTK_LIBRARY_DIRS})
add_definitions(${GTK_CFLAGS_OTHER})

target_link_libraries(my_app ${GTK_LIBRARIES})
