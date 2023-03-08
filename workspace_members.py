#!/usr/bin/python3

## workspace内のcrateのCargo.tomlを参照してpackage > nameがあれば
## 列挙してrootのCargo.tomlに書き出す。

## tomlモジュールを利用するのでインストールする。
## pip install toml 或いは python -m pip install toml

import toml
import glob

def main():
    targets = glob.glob("./**/Cargo.toml")
    buf = {"workspace": { "members" : [] }}

    for target in targets:
        with open(target) as rf:
            obj = toml.load(rf)
            print("> %s" % obj["package"]["name"])
            buf["workspace"]["members"].append(obj["package"]["name"])

    with open("Cargo.toml", "w", encoding="UTF-8") as fw:
        toml.dump(buf, fw)

if __name__ == "__main__":
    main()
    print("done.")

