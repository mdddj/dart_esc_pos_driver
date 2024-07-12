cmake_minimum_required(VERSION 3.10)
set(PROJECT_NAME "dart_esc_pos_driver")
project(${PROJECT_NAME} LANGUAGES CXX)
include("../cargokit/cmake/cargokit.cmake")
apply_cargokit(${PROJECT_NAME} ../rust dart_esc_pos_driver "")
set(dart_esc_pos_driver_bundled_libraries
  "${${PROJECT_NAME}_cargokit_lib}"
  PARENT_SCOPE
)