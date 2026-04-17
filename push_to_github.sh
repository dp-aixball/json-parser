#!/bin/bash

echo "=== 推送JSON解析器项目到GitHub ==="
echo ""

# 检查是否在Git仓库中
if [ ! -d ".git" ]; then
    echo "错误：当前目录不是Git仓库"
    exit 1
fi

echo "1. 检查远程仓库配置..."
if git remote | grep -q "origin"; then
    echo "✅ 远程仓库已配置"
    git remote -v
else
    echo "❌ 未配置远程仓库"
    echo ""
    echo "请先在GitHub上创建仓库，然后运行以下命令："
    echo "git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO.git"
    echo ""
    echo "或者，如果您想让我帮您配置，请输入GitHub仓库URL："
    read -p "GitHub仓库URL: " repo_url
    if [ -n "$repo_url" ]; then
        git remote add origin "$repo_url"
        echo "✅ 已添加远程仓库：$repo_url"
    else
        echo "❌ 未提供URL，退出"
        exit 1
    fi
fi

echo ""
echo "2. 重命名分支为main（如果需要）..."
current_branch=$(git branch --show-current)
if [ "$current_branch" = "master" ]; then
    echo "当前分支是master，重命名为main..."
    git branch -M main
    echo "✅ 分支已重命名为main"
else
    echo "当前分支：$current_branch"
fi

echo ""
echo "3. 推送到GitHub..."
echo "推送分支到origin/main..."
git push -u origin main

echo ""
echo "4. 验证推送结果..."
git log --oneline -3
echo ""
echo "远程仓库状态："
git remote show origin

echo ""
echo "✅ 推送完成！"
echo "📦 您的项目现在在GitHub上："
git remote get-url origin