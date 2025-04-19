cmake_minimum_required(VERSION 3.10)

project(lru_cache)

include_directories(
  /Library/Developer/CommandLineTools/SDKs/MacOSX15.1.sdk/usr/include/c++/v1
)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "-fsanitize=address")
add_executable(lru_cache lru_cache.cc)
