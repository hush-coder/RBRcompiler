# 内存：8GB以上
# 操作系统：Ubuntu 20.04
# 目标架构：AArch64
# GCC版本：4.8.5以上
# glibc版本：2.17以上
# libatomic版本：1.2及以上
# libstdc++版本：6及以上

# 使用 Ubuntu 20.04 作为基础镜像
FROM ubuntu:20.04

# 镜像作者信息
LABEL maintainer="your_name@example.com"

# 设置工作目录
WORKDIR /app

# 安装必要的软件包，包括 dos2unix 工具、qemu 模拟器和 Python3
# 将所有命令合并以减少镜像层数
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
    gcc g++ make git curl wget ca-certificates dos2unix python3 python3-pip \
    libatomic-ops-dev libstdc++-10-dev qemu-user-static && \
    rm -rf /var/lib/apt/lists/* && \
    gcc --version && \
    ld --version

# 安装 Rust
# 使用 rustup 官方脚本安装 Rust，并设置 cargo 路径
# 使用 -y 选项跳过交互式安装
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $HOME/.cargo/env

# 创建毕昇编译器安装目录
RUN mkdir -p /opt/compiler

# 将毕昇编译器软件包拷贝到安装目录下
# 确保 BiShengCompiler-4.2.0.2-aarch64-linux.tar.gz 和 Dockerfile 在同一目录下
COPY BiShengCompiler-4.2.0.2-aarch64-linux.tar.gz /opt/compiler/

# 解压毕昇编译器软件包
WORKDIR /opt/compiler
RUN tar -zxvf BiShengCompiler-4.2.0.2-aarch64-linux.tar.gz && \
    chmod -R 755 BiShengCompiler-4.2.0.2-aarch64-linux

# 将你的项目文件拷贝到镜像中
COPY . /app

# 自动转换所有 .sh 脚本的行尾符，确保在 Linux 环境中可以正常运行
RUN dos2unix /app/compiler/scripts/*.sh

# 设置环境变量，包括毕昇和 Rust
ENV BISHENG_HOME="/opt/compiler/BiShengCompiler-4.2.0.2-aarch64-linux"
ENV PATH="${BISHENG_HOME}/bin:${HOME}/.cargo/bin:${PATH}"
ENV LLVM_BIN="${BISHENG_HOME}/bin"
ENV LD_LIBRARY_PATH="${BISHENG_HOME}/lib:${BISHENG_HOME}/lib/aarch64-unknown-linux-gnu:${LD_LIBRARY_PATH}"

# 在这里添加安装 vim 的命令
RUN apt-get update -y && apt-get install -y vim && \
    rm -rf /var/lib/apt/lists/*

# 设置默认命令，当容器启动时执行
WORKDIR /app/compiler
CMD ["bash"]