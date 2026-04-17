#!/bin/bash

echo "=== Git提交JSON解析器项目 ==="
echo ""

# 检查是否在Git仓库中
if [ ! -d ".git" ]; then
    echo "错误：当前目录不是Git仓库"
    exit 1
fi

echo "1. 添加所有文件到暂存区..."
git add .

echo ""
echo "2. 查看暂存区状态..."
git status

echo ""
echo "3. 提交更改..."
git commit -m "feat: 添加Rust JSON解析器项目

- 创建两个版本的JSON解析器：
  * json_parser: 详细输出格式
  * simple_json_parser: 简化输出格式，支持命令行参数
- 添加示例JSON文件：
  * data.json: 基本示例
  * test.json: 测试数据
  * complex.json: 复杂测试数据
- 添加测试脚本 test_simple.sh
- 配置Cargo.toml支持多个二进制目标
- 添加.gitignore排除target目录"

echo ""
echo "4. 查看提交历史..."
git log --oneline -5

echo ""
echo "✅ Git提交完成！"