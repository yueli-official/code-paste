<script setup lang="ts"></script>

<template>
  <!-- Hero 区域 -->
  <section class="relative py-20 px-4 overflow-hidden">
    <!-- 背景装饰 -->
    <div
      class="absolute inset-0 bg-gradient-to-br from-primary/10 via-transparent to-secondary/5"
    ></div>
    <div
      class="absolute top-0 left-1/4 w-72 h-72 bg-primary/10 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob"
    ></div>
    <div
      class="absolute top-0 right-1/4 w-72 h-72 bg-secondary/10 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob animation-delay-2000"
    ></div>
    <div
      class="absolute -bottom-8 left-1/3 w-72 h-72 bg-accent/10 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob animation-delay-4000"
    ></div>

    <div class="container mx-auto max-w-6xl relative z-10">
      <div class="text-center space-y-8">
        <!-- 主标题 -->
        <div class="space-y-4">
          <h1
            class="text-4xl md:text-6xl lg:text-7xl font-bold bg-gradient-to-r from-foreground via-primary to-foreground bg-clip-text text-transparent leading-tight"
          >
            代码分享
            <span class="block text-primary">更简单</span>
          </h1>
          <p class="text-xl md:text-2xl text-muted-foreground max-w-3xl mx-auto leading-relaxed">
            快速分享代码片段，支持语法高亮、一键复制，让代码协作变得轻松愉快
          </p>
        </div>

        <!-- CTA 按钮 -->
        <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
          <a
            href="/new"
            class="group relative inline-flex items-center justify-center px-8 py-4 text-lg font-semibold text-primary-foreground bg-primary rounded-full hover:bg-primary/90 transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-primary/25"
          >
            <svg
              class="w-5 h-5 mr-2 group-hover:rotate-12 transition-transform"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 4v16m8-8H4"
              ></path>
            </svg>
            开始分享代码
            <div
              class="absolute inset-0 rounded-full bg-white/20 scale-0 group-hover:scale-100 transition-transform duration-300"
            ></div>
          </a>

          <a
            href="#features"
            class="inline-flex items-center justify-center px-8 py-4 text-lg font-semibold text-foreground bg-background border-2 border-border rounded-full hover:bg-accent hover:border-primary/50 transition-all duration-300"
          >
            了解更多
            <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 14l-7 7m0 0l-7-7m7 7V3"
              ></path>
            </svg>
          </a>
        </div>

        <!-- 统计数据 -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8 pt-12">
          <div class="text-center">
            <div class="text-3xl md:text-4xl font-bold text-primary">10K+</div>
            <div class="text-sm text-muted-foreground mt-1">代码片段</div>
          </div>
          <div class="text-center">
            <div class="text-3xl md:text-4xl font-bold text-primary">50+</div>
            <div class="text-sm text-muted-foreground mt-1">编程语言</div>
          </div>
          <div class="text-center">
            <div class="text-3xl md:text-4xl font-bold text-primary">5K+</div>
            <div class="text-sm text-muted-foreground mt-1">活跃用户</div>
          </div>
          <div class="text-center">
            <div class="text-3xl md:text-4xl font-bold text-primary">99.9%</div>
            <div class="text-sm text-muted-foreground mt-1">可用性</div>
          </div>
        </div>
      </div>
    </div>
  </section>

  <!-- 功能特性 -->
  <section id="features" class="py-20 px-4 bg-muted/30">
    <div class="container mx-auto max-w-6xl">
      <div class="text-center space-y-4 mb-16">
        <h2 class="text-3xl md:text-5xl font-bold">为什么选择我们？</h2>
        <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
          专为开发者设计的代码分享平台，提供最佳的使用体验
        </p>
      </div>

      <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
        <!-- 语法高亮 -->
        <div
          class="group p-8 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300"
        >
          <div
            class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary/20 transition-colors"
          >
            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
              ></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold mb-3">智能语法高亮</h3>
          <p class="text-muted-foreground leading-relaxed">
            支持 50+ 种编程语言的语法高亮，自动识别代码类型，让你的代码更易读
          </p>
        </div>

        <!-- 一键复制 -->
        <div
          class="group p-8 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300"
        >
          <div
            class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary/20 transition-colors"
          >
            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
              ></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold mb-3">一键复制</h3>
          <p class="text-muted-foreground leading-relaxed">
            点击即可复制整个代码片段，支持格式保持，提升开发效率
          </p>
        </div>

        <!-- 私密分享 -->
        <div
          class="group p-8 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300"
        >
          <div
            class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary/20 transition-colors"
          >
            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
              ></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold mb-3">隐私保护</h3>
          <p class="text-muted-foreground leading-relaxed">
            支持私密分享，设置密码保护，确保你的敏感代码安全无忧
          </p>
        </div>

        <!-- 过期设置 -->
        <div
          class="group p-8 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300"
        >
          <div
            class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary/20 transition-colors"
          >
            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
              ></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold mb-3">自动过期</h3>
          <p class="text-muted-foreground leading-relaxed">
            灵活的过期时间设置，从1小时到永不过期，满足不同使用场景
          </p>
        </div>

        <!-- 多种格式 -->
        <div
          class="group p-8 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300"
        >
          <div
            class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary/20 transition-colors"
          >
            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
              ></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold mb-3">多种格式</h3>
          <p class="text-muted-foreground leading-relaxed">
            支持纯文本、Markdown、JSON等多种格式，适配各种使用需求
          </p>
        </div>

        <!-- 暗色模式 -->
        <div
          class="group p-8 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300"
        >
          <div
            class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary/20 transition-colors"
          >
            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
              ></path>
            </svg>
          </div>
          <h3 class="text-xl font-semibold mb-3">暗色模式</h3>
          <p class="text-muted-foreground leading-relaxed">
            完美适配明暗主题，保护眼睛，让深夜编程更加舒适
          </p>
        </div>
      </div>
    </div>
  </section>

  <!-- 使用流程 -->
  <section class="py-20 px-4">
    <div class="container mx-auto max-w-6xl">
      <div class="text-center space-y-4 mb-16">
        <h2 class="text-3xl md:text-5xl font-bold">三步轻松分享</h2>
        <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
          简单三步，快速分享你的代码片段
        </p>
      </div>

      <div class="grid md:grid-cols-3 gap-8">
        <!-- 步骤 1 -->
        <div class="relative text-center group">
          <div
            class="w-16 h-16 bg-primary rounded-full flex items-center justify-center text-primary-foreground text-2xl font-bold mx-auto mb-6 group-hover:scale-110 transition-transform duration-300"
          >
            1
          </div>
          <h3 class="text-xl font-semibold mb-3">粘贴代码</h3>
          <p class="text-muted-foreground leading-relaxed">
            将你的代码粘贴到编辑器中，设置编程语言
          </p>
          <!-- 连接线 -->
          <div
            class="hidden md:block absolute top-8 left-full w-full h-px bg-gradient-to-r from-primary to-transparent -z-10"
          ></div>
        </div>

        <!-- 步骤 2 -->
        <div class="relative text-center group">
          <div
            class="w-16 h-16 bg-primary rounded-full flex items-center justify-center text-primary-foreground text-2xl font-bold mx-auto mb-6 group-hover:scale-110 transition-transform duration-300"
          >
            2
          </div>
          <h3 class="text-xl font-semibold mb-3">设置选项</h3>
          <p class="text-muted-foreground leading-relaxed">
            选择过期时间、设置密码保护，自定义分享偏好
          </p>
          <!-- 连接线 -->
          <div
            class="hidden md:block absolute top-8 left-full w-full h-px bg-gradient-to-r from-primary to-transparent -z-10"
          ></div>
        </div>

        <!-- 步骤 3 -->
        <div class="text-center group">
          <div
            class="w-16 h-16 bg-primary rounded-full flex items-center justify-center text-primary-foreground text-2xl font-bold mx-auto mb-6 group-hover:scale-110 transition-transform duration-300"
          >
            3
          </div>
          <h3 class="text-xl font-semibold mb-3">一键分享</h3>
          <p class="text-muted-foreground leading-relaxed">
            点击分享按钮，获取链接，轻松与他人分享
          </p>
        </div>
      </div>
    </div>
  </section>

  <!-- CTA 区域 -->
  <section class="py-20 px-4">
    <div class="container mx-auto max-w-4xl text-center">
      <div class="space-y-8">
        <div class="space-y-4">
          <h2 class="text-3xl md:text-5xl font-bold">准备开始了吗？</h2>
          <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
            加入我们的社区，与全球开发者分享你的代码创意
          </p>
        </div>

        <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
          <a
            href="/new"
            class="group relative inline-flex items-center justify-center px-8 py-4 text-lg font-semibold text-primary-foreground bg-primary rounded-full hover:bg-primary/90 transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-primary/25"
          >
            立即开始分享
            <svg
              class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13 7l5 5m0 0l-5 5m5-5H6"
              ></path>
            </svg>
          </a>

          <a
            href="/about"
            class="inline-flex items-center justify-center px-8 py-4 text-lg font-semibold text-muted-foreground hover:text-foreground transition-colors"
          >
            了解更多信息
          </a>
        </div>
      </div>
    </div>
  </section>
</template>

<style>
/* 浮动动画 */
@keyframes blob {
  0% {
    transform: translate(0px, 0px) scale(1);
  }
  33% {
    transform: translate(30px, -50px) scale(1.1);
  }
  66% {
    transform: translate(-20px, 20px) scale(0.9);
  }
  100% {
    transform: translate(0px, 0px) scale(1);
  }
}

.animate-blob {
  animation: blob 7s infinite;
}

.animation-delay-2000 {
  animation-delay: 2s;
}

.animation-delay-4000 {
  animation-delay: 4s;
}

/* 平滑滚动 */
html {
  scroll-behavior: smooth;
}

/* 减少动画偏好 */
@media (prefers-reduced-motion: reduce) {
  .animate-blob {
    animation: none;
  }

  * {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}
</style>
