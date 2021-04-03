cmake_minimum_required(VERSION 3.17)
project(practice_dp)

set(CMAKE_CXX_STANDARD 17)

include_directories($ENV{GHQ_ROOT}/github.com/atcoder/ac-library)

add_executable(practice_dp_1_maximum_sum 1_maximum_sum.cpp)
