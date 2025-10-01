


# RBRcompiler

---

## 一、准备工作

1. **下载软件包**：
   - 点击[链接](https://www.hikunpeng.com/developer/devkit/download/bishengcompiler)，下载软件包 `BiShengCompiler-4.2.0.2-aarch64-linux.tar.gz`。（因为 GitHub 上传大文件挺麻烦的）
   - 将下载好的软件包放入 `RBRcompiler` 文件夹中，确保它与 `compiler` 文件夹处于同一层级。

2. **配置 Docker**：
   - 前置条件：确保你已经配置好 Docker。

3. **构建镜像**：
   - 从终端进入 `RBRcompiler` 文件夹。
   - 输入以下命令构建镜像：
     ```bash
     docker build --platform linux/arm64 -t my-compiler-app-3.0 .
     ```
   - 构建完成后，输入以下命令查看现存的镜像：
     ```bash
     docker images
     ```

## 二、运行试验 1

### 1. 运行容器

- 输入以下命令运行容器：
  ```bash
  docker run -it my-compiler-app-3.0:latest /bin/bash
  ```

### 2. 进入脚本文件目录

- 输入以下命令进入脚本文件目录：
  ```bash
  cd tests/lexer/
  ```

### 3. 运行脚本

- 由于可能出现格式问题，需要将 `.sh` 文件转换成正确的格式。执行以下命令：
  ```bash
  dos2unix ./simple_test.sh
  ```
  - 如果成功，会显示：
    ```
    dos2unix: converting file ./simple_test.sh to Unix format...
    ```
- 输入以下命令运行脚本：
  ```bash
  ./simple_test.sh
  ```
- 最后，实验的结果会显示在终端中。

### 4. 实验 2（AST 生成）

类似实验 1，按顺序运行如下命令：

```bash
cd tests/parser
dos2unix ./run_all.sh
./run_all.sh
```

你就会在 `output` 目录下看到两个生成的文件。

---
```

