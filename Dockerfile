# ----------------------
# 前端构建阶段
# ----------------------
FROM node:20-alpine AS frontend-builder
WORKDIR /frontend

# 安装 pnpm
RUN npm install -g pnpm

# 复制前端依赖文件
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile && pnpm store prune

# 复制前端源码并构建
COPY frontend/ .
RUN pnpm build

# ----------------------
# 后端构建阶段
# ----------------------
FROM rust:1.89 AS backend-builder
WORKDIR /usr/src/app

# 设置默认数据库路径
ENV DATABASE_URL=/app/snippets.db

# 安装 musl 工具链
RUN apt-get update && apt-get install -y musl-tools build-essential

# 添加 musl target (使用当前架构)
RUN rustup target add $(rustc -vV | grep host | cut -d' ' -f2 | sed 's/gnu/musl/')

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "// placeholder" > src/main.rs

# 复制源码
COPY src ./src

# 构建 release 静态可执行文件 (使用当前架构的 musl target)
RUN cargo build --release --target $(rustc -vV | grep host | cut -d' ' -f2 | sed 's/gnu/musl/')

# 创建符号链接以便后续阶段能找到可执行文件
RUN ln -s target/$(rustc -vV | grep host | cut -d' ' -f2 | sed 's/gnu/musl/')/release/paste_backend ./paste_backend

# ----------------------
# 运行阶段
# ----------------------
FROM alpine:latest

# 安装 SQLite 运行时
RUN apk add --no-cache sqlite

WORKDIR /app

# 复制后端可执行文件
COPY --from=backend-builder /usr/src/app/paste_backend ./

# 复制前端构建结果到后端能访问的位置
COPY --from=frontend-builder /frontend/dist ./frontend/dist

# 暴露端口
EXPOSE 9000

# 默认启动命令
CMD ["./paste_backend"]