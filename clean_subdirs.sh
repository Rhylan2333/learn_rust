#!/bin/bash

# 获取当前路径
current_dir=$(pwd)

# 获取当前路径下的所有子目录
subdirs=$(find . -type d)

# 遍历每个子目录并执行 cargo clean
for subdir in $subdirs; do
    if [ "$subdir" != "." ]; then
        cd "$current_dir/$subdir"
        echo "Cleaning $subdir..."
        cargo clean
    fi
done