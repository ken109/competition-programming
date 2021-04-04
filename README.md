# 環境設定
### 依存関係
```bash
brew install gcc

ghq get git@github.com:atcoder/ac-library.git
```

### CLion
* Cコンパイラ
  * /usr/local/opt/gcc/bin/gcc-10
* C++コンパイラ
  * /usr/local/opt/gcc/bin/g++-10

# ディレクトリ構成
at_coderのみならず、プロジェクトルート(このレポジトリの一階層め)にあるディレクトリは全てプロジェクトとみなされる。  
そのプロジェクトのルートにあるディレクトリ(このレポジトリの二階層め)は全てコンテストとみなされる。

この制約を守っていないと、`gen_build_settings.sh`を実行できない


# 使い方

```bash
cd at_coder

# コンテストディレクトリを作成する
mkdir abc123
```

### Python
1. pythonディレクトリを作成する
2. `.py`ファイルを作成する
3. コーディングする
4. 実行する

### C++
1. c++ディレクトリを作成する
2. `.cpp`ファイルを作成する
3. `./gen_build_settings.sh`を実行する
4. コーディングする
5. 実行する

### Rust
1. rustディレクトリを作成する
2. `.rs`ファイルを作成する
3. `./gen_build_settings.sh`を実行する
4. コーディングする
5. 実行する
