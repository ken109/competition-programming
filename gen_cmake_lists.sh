#!/usr/bin/env bash

set -eu

while IFS= read -r -d '' project_dir; do
  while IFS= read -r -d '' contest_dir; do
    if [[ $contest_dir == *cmake-build-debug ]]; then
      continue
    fi

    project=$(echo "$contest_dir" | awk '{split($0, ary, "/"); print ary[6]}')
    contest=$(echo "$contest_dir" | awk '{split($0, ary, "/"); print ary[7]}')

    {
      echo "cmake_minimum_required(VERSION 3.17)"
      echo "project(${project}_${contest})"
      echo ""
      echo "set(CMAKE_CXX_STANDARD 17)"
      echo ""
      echo "include_directories(\$ENV{GHQ_ROOT}/github.com/atcoder/ac-library)"
      echo ""
    } >"$contest_dir/CMakeLists.txt"

    while IFS= read -r -d '' cpp_path; do
      cpp_file_name=$(echo "$cpp_path" | awk '{split($0, ary, "/"); print ary[8]}')
      cpp_name=$(echo "$cpp_file_name" | awk '{split($0, ary, "."); print ary[1]}')
      echo "add_executable(${project}_${contest}_${cpp_name} ${cpp_file_name})" >>"$contest_dir/CMakeLists.txt"
    done < <(find "$contest_dir" -name "*.cpp" -type f -maxdepth 1 -print0)
  done < <(find "$project_dir/"* -type d -maxdepth 1 -print0)
done < <(find "$(
  cd "$(dirname "$0")" || exit
  pwd
)/"* -type d -maxdepth 0 -print0)
