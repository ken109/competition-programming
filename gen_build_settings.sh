#!/usr/bin/env bash

set -eu

while IFS= read -r -d '' project_dir; do
  while IFS= read -r -d '' contest_dir; do
    project=$(echo "$contest_dir" | awk '{split($0, ary, "/"); print ary[6]}')
    contest=$(echo "$contest_dir" | awk '{split($0, ary, "/"); print ary[7]}')

    # ------------------------- C++ -------------------------
    if [[ -d "$contest_dir/c++" ]]; then
      {
        echo "cmake_minimum_required(VERSION 3.17)"
        echo "project(${project}_${contest})"
        echo ""
        echo "set(CMAKE_CXX_STANDARD 17)"
        echo ""
        echo "include_directories(\$ENV{GHQ_ROOT}/github.com/atcoder/ac-library)"
        echo ""
      } >"$contest_dir/c++/CMakeLists.txt"

      while IFS= read -r -d '' cpp_path; do
        {
          echo "add_executable(${project}_${contest}_$(basename "$cpp_path" .cpp) $(basename "$cpp_path"))"
        } >>"$contest_dir/c++/CMakeLists.txt"
      done < <(find "$contest_dir/c++" -name "*.cpp" -type f -maxdepth 1 -print0)
    fi

    # ------------------------- Rust -------------------------
    if [[ -d "$contest_dir/rust" ]]; then
      {
        echo "[package]"
        echo "name = \"${project}_${contest}\""
        echo "version = \"1.0.0\""
        echo "edition = \"2018\""
        echo "authors = [\"Kensuke Kubo <kensukekubo19@gmail.com>\"]"
        echo ""
        echo "[dependencies]"
        echo "proconio = { version = \"0.4.2\", features = [\"derive\"] }"
        echo ""
      } >"$contest_dir/rust/Cargo.toml"

      while IFS= read -r -d '' rust_path; do
        {
          echo "[[bin]]"
          echo "name = \"$(basename "$rust_path" .rs)\""
          echo "path = \"$(basename "$rust_path")\""
          echo ""
        } >>"$contest_dir/rust/Cargo.toml"
      done < <(find "$contest_dir/rust" -name "*.rs" -type f -maxdepth 1 -print0)
    fi

  done < <(find "$project_dir/"* -type d -maxdepth 0 -print0)
done < <(find "$(
  cd "$(dirname "$0")" || exit
  pwd
)/"* -type d -maxdepth 0 -print0)
