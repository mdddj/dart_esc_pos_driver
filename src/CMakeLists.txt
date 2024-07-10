# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(dart_esc_pos_driver_library VERSION 0.0.1 LANGUAGES C)

add_library(dart_esc_pos_driver SHARED
  "dart_esc_pos_driver.c"
)

set_target_properties(dart_esc_pos_driver PROPERTIES
  PUBLIC_HEADER dart_esc_pos_driver.h
  OUTPUT_NAME "dart_esc_pos_driver"
)

target_compile_definitions(dart_esc_pos_driver PUBLIC DART_SHARED_LIB)
