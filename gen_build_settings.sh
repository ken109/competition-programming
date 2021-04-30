#!/usr/bin/env bash

set -eu

root=$(
  cd "$(dirname "$0")" || exit
  pwd
)

{
  echo "cmake_minimum_required(VERSION 3.17)"
  echo "project(ken109_procon)"
  echo ""
  echo "set(CMAKE_CXX_STANDARD 17)"
  echo ""
  echo "include_directories(\$ENV{GHQ_ROOT}/github.com/atcoder/ac-library)"
  echo ""
} >"$root/CMakeLists.txt"

{
  echo "[package]"
  echo "name = \"ken109_procon\""
  echo "version = \"1.0.0\""
  echo "edition = \"2018\""
  echo "authors = [\"Kensuke Kubo <kensukekubo19@gmail.com>\"]"
  echo ""
  echo "[dependencies]"
  echo "num = \"=0.2.1\""
  echo "num-bigint = \"=0.2.6\""
  echo "num-complex = \"=0.2.4\""
  echo "num-integer = \"=0.1.42\""
  echo "num-iter = \"=0.1.40\""
  echo "num-rational = \"=0.2.4\""
  echo "num-traits = \"=0.2.11\""
  echo "num-derive = \"=0.3.0\""
  echo "ndarray = \"=0.13.0\""
  echo "nalgebra = \"=0.20.0\""
  echo "alga = \"=0.9.3\""
  echo "libm = \"=0.2.1\""
  echo "rand = { version = \"=0.7.3\", features = [\"small_rng\"] }"
  echo "getrandom = \"=0.1.14\""
  echo "rand_chacha = \"=0.2.2\""
  echo "rand_core = \"=0.5.1\""
  echo "rand_hc = \"=0.2.0\""
  echo "rand_pcg = \"=0.2.1\""
  echo "rand_distr = \"=0.2.2\""
  echo "petgraph = \"=0.5.0\""
  echo "indexmap = \"=1.3.2\""
  echo "regex = \"=1.3.6\""
  echo "lazy_static = \"=1.4.0\""
  echo "ordered-float = \"=1.0.2\""
  echo "ascii = \"=1.0.0\""
  echo "permutohedron = \"=0.2.4\""
  echo "superslice = \"=1.0.0\""
  echo "itertools = \"=0.9.0\""
  echo "itertools-num = \"=0.1.3\""
  echo "maplit = \"=1.0.2\""
  echo "either = \"=1.5.3\""
  echo "im-rc = \"=14.3.0\""
  echo "fixedbitset = \"=0.2.0\""
  echo "bitset-fixed = \"=0.1.0\""
  echo "proconio = { version = \"=0.3.6\", features = [\"derive\"] }"
  echo "text_io = \"=0.1.8\""
  echo "whiteread = \"=0.5.0\""
  echo "rustc-hash = \"=1.1.0\""
  echo "smallvec = \"=1.2.0\""
  echo ""
} >"$root/Cargo.toml"

while IFS= read -r -d '' project_dir; do
  while IFS= read -r -d '' contest_dir; do
    project=$(echo "$contest_dir" | awk '{split($0, ary, "/"); print ary[6]}')
    contest=$(echo "$contest_dir" | awk '{split($0, ary, "/"); print ary[7]}')

    # ------------------------- C++ -------------------------
    if [[ -d "$contest_dir/c++" ]]; then
      while IFS= read -r -d '' cpp_path; do
        {
          echo "add_executable(${project}_${contest}_$(basename "$cpp_path" .cpp) ${project}/${contest}/c++/$(basename "$cpp_path"))"
        } >>"$root/CMakeLists.txt"
      done < <(find "$contest_dir/c++" -name "*.cpp" -type f -maxdepth 1 -print0)
    fi

    # ------------------------- Rust -------------------------
    if [[ -d "$contest_dir/rust" ]]; then

      while IFS= read -r -d '' rust_path; do
        {
          echo "[[bin]]"
          echo "name = \"${project}_${contest}_$(basename "$rust_path" .rs)\""
          echo "path = \"${project}/${contest}/rust/$(basename "$rust_path")\""
          echo ""
        } >>"$root/Cargo.toml"
      done < <(find "$contest_dir/rust" -name "*.rs" -type f -maxdepth 1 -print0)
    fi

  done < <(find "$project_dir/"* -type d -maxdepth 0 -print0)
done < <(find "$root/"* -type d -maxdepth 0 -print0)
