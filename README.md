


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
    docker build --platform linux/arm64 -t rbr-compiler:latest .
     ```
   - 构建完成后，输入以下命令查看现存的镜像：
     ```bash
     docker images
     ```

## 二、运行试验 1

### 1. 运行容器

  ```bash
 docker run --rm -it -v <这里填Dockfile所在的文件>:/app rbr-compiler:latest bash
  ```

### 2. 进入脚本文件目录

  ```bash
  cd tests/lexer/
  ```

### 3. 运行脚本

  ```bash
  dos2unix ./simple_test.sh
  ```
  - 如果成功，会显示：
    ```
    dos2unix: converting file ./simple_test.sh to Unix format...
    ```
  ```bash
  ./simple_test.sh
  ```

### 4. 实验 2（AST 生成）

类似实验 1，按顺序运行如下命令：

```bash
cd tests/parser
dos2unix ./run_all.sh
./run_all.sh
```

你就会在 `output` 目录下看到两个生成的文件。

```
## 三、命令行（CLI）使用（简洁）



参数说明（简洁）：

- `FILE`：输入 C 源文件（必需）
- `--format, -f`：tokens 输出格式，`text`（默认）或 `json`
- `--emit <tokens|semantic>`：以 bisheng 格式导出（支持 `tokens`、`semantic`）
- `--ast-bisheng`：配合 `--ast-output`，以 bisheng 格式输出 AST
- `--emit-ir`：输出 IR（中间表示）
- `--emit-arm`：输出 ARM 汇编
- `--output, -o`：写入输出文件；不指定则打印到 stdout
- `--ast-output`：将 AST 写入指定文件

更多细节请查看 `compiler/src/main.rs`。

