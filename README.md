# 🍳 吃了么

> **今天吃点什么好呢？** 让选择困难症不再困难！

一款轻量级的家庭菜谱展示应用，用 **Rust** 打造，极速启动，告别"吃什么"的世纪难题。

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green.svg)

---

## ✨ 特色功能

### 🎴 精美卡片展示
每道菜都是一张精心设计的卡片，点击即可查看完整大图，让美食先饱眼福。

### 🏷️ 智能标签筛选
荤菜、素菜、小炒、汤类... 一键筛选，快速找到今天想吃的类型。

### 🔍 快速搜索
想吃红烧肉？直接搜！模糊匹配，秒出结果。

### 🎲 摇号决定
选择困难症的终极解决方案！输入数字，随机摇出今天的菜单，命运帮你做决定。

---

## 🚀 快速开始

```bash
# 编译
cargo build --release

# 运行
./target/release/recipe

# 打开浏览器访问
open http://localhost:3000
```

---

## 📁 添加新菜谱

1. 创建以菜名命名的文件夹（如 `糖醋里脊/`）
2. 放入菜谱图片，命名为 `z-image-xxx.png`
3. 在 `tags.json` 中添加标签配置
4. 重启服务，新菜谱自动加载！

---

## 🛠️ 技术栈

- **后端**: Rust + Axum（高性能异步框架）
- **模板**: Tera（灵活的模板引擎）
- **前端**: 原生 HTML/CSS/JS（零依赖，极速加载）

---

## 🙏 感谢

- 🤖 [豆包](https://www.doubao.com/) - 菜谱灵感
- 🎨 [Z-Image](https://www.modelscope.cn/models/Tongyi-MAI/Z-Image-Turbo) - AI 图片生成
- 📕 小红书 - 美食参考

---

<p align="center">
  <b>🍜 美食让生活更美好</b><br>
  <sub>Made with ❤️ and Rust</sub>
</p>
