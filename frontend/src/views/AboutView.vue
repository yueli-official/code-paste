<template>
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

    <div class="container mx-auto max-w-6xl relative z-10">
      <div class="text-center space-y-8">
        <div class="space-y-4">
          <h1
            class="text-3xl md:text-6xl lg:text-7xl font-bold bg-gradient-to-r from-foreground via-primary to-foreground bg-clip-text text-transparent leading-tight"
          >
            关于我们
          </h1>
          <p class="text-xl md:text-2xl text-muted-foreground max-w-3xl mx-auto leading-relaxed">
            致力于为开发者提供最优质的代码分享体验
          </p>
        </div>
      </div>
    </div>
  </section>

  <section class="py-20 px-4 bg-muted/30">
    <div class="container mx-auto max-w-6xl">
      <div class="grid md:grid-cols-2 gap-12 items-center">
        <div class="space-y-8">
          <div class="space-y-4">
            <h2 class="text-3xl md:text-4xl font-bold">我们想做的事</h2>
            <p class="text-lg text-muted-foreground leading-relaxed">
              我们希望让代码分享更轻松、更安全、更高效。开发者应该专注于创造，而不是被复杂的流程拖累。
            </p>
          </div>

          <div class="space-y-4">
            <h3 class="text-2xl font-semibold">我们坚持的原则</h3>
            <div class="space-y-3">
              <div class="flex items-start space-x-3">
                <div class="w-2 h-2 bg-primary rounded-full mt-3 flex-shrink-0"></div>
                <div>
                  <h4 class="font-semibold">简洁</h4>
                  <p class="text-muted-foreground">去繁就简，让功能直观易用，每一步都物有所值。</p>
                </div>
              </div>
              <div class="flex items-start space-x-3">
                <div class="w-2 h-2 bg-primary rounded-full mt-3 flex-shrink-0"></div>
                <div>
                  <h4 class="font-semibold">安全</h4>
                  <p class="text-muted-foreground">保护隐私与数据安全是我们的底线，不打折扣。</p>
                </div>
              </div>
              <div class="flex items-start space-x-3">
                <div class="w-2 h-2 bg-primary rounded-full mt-3 flex-shrink-0"></div>
                <div>
                  <h4 class="font-semibold">开放</h4>
                  <p class="text-muted-foreground">让知识自由流动，让开发者更好地交流与合作。</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="relative">
          <div
            class="aspect-square bg-gradient-to-br from-primary/20 to-secondary/20 rounded-3xl p-8 flex items-center justify-center"
          >
            <div class="text-center space-y-6">
              <div
                class="w-24 h-24 bg-primary/10 rounded-2xl flex items-center justify-center mx-auto"
              >
                <svg
                  class="w-12 h-12 text-primary"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
                  ></path>
                </svg>
              </div>
              <div class="space-y-2">
                <h3 class="text-2xl font-bold">创新驱动</h3>
                <p class="text-muted-foreground">持续改进，拥抱变化</p>
              </div>
            </div>
          </div>
          <!-- 浮动装饰元素 -->
          <div
            class="absolute -top-4 -right-4 w-8 h-8 bg-primary/20 rounded-full animate-ping"
          ></div>
          <div
            class="absolute -bottom-4 -left-4 w-6 h-6 bg-secondary/30 rounded-full animate-pulse"
          ></div>
        </div>
      </div>
    </div>
  </section>

  <!-- 技术栈 -->
  <section class="py-20 px-4">
    <div class="container mx-auto max-w-6xl">
      <div class="text-center space-y-4 mb-16">
        <h2 class="text-3xl md:text-5xl font-bold">技术架构</h2>
        <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
          采用现代化技术栈，保障平台稳定高效运行
        </p>
      </div>

      <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
        <!-- 前端 -->
        <div
          class="text-center p-6 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300 group"
        >
          <div
            class="w-16 h-16 bg-blue-500/10 rounded-2xl flex items-center justify-center mx-auto mb-4 group-hover:bg-blue-500/20 transition-colors"
          >
            <svg class="w-8 h-8 text-blue-500" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>
            </svg>
          </div>
          <h3 class="font-semibold mb-2">前端技术</h3>
          <p class="text-sm text-muted-foreground">Vue, TypeScript, Tailwind CSS</p>
        </div>

        <!-- 后端 -->
        <div
          class="text-center p-6 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300 group"
        >
          <div
            class="w-16 h-16 bg-green-500/10 rounded-2xl flex items-center justify-center mx-auto mb-4 group-hover:bg-green-500/20 transition-colors"
          >
            <svg class="w-8 h-8 text-green-500" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
              ></path>
            </svg>
          </div>
          <h3 class="font-semibold mb-2">后端服务</h3>
          <p class="text-sm text-muted-foreground">Rust</p>
        </div>

        <!-- 存储 -->
        <div
          class="text-center p-6 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300 group"
        >
          <div
            class="w-16 h-16 bg-purple-500/10 rounded-2xl flex items-center justify-center mx-auto mb-4 group-hover:bg-purple-500/20 transition-colors"
          >
            <svg
              class="w-8 h-8 text-purple-500"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"
              ></path>
            </svg>
          </div>
          <h3 class="font-semibold mb-2">数据存储</h3>
          <p class="text-sm text-muted-foreground">Sqlite</p>
        </div>

        <!-- 部署 -->
        <div
          class="text-center p-6 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300 group"
        >
          <div
            class="w-16 h-16 bg-orange-500/10 rounded-2xl flex items-center justify-center mx-auto mb-4 group-hover:bg-orange-500/20 transition-colors"
          >
            <svg
              class="w-8 h-8 text-orange-500"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
              ></path>
            </svg>
          </div>
          <h3 class="font-semibold mb-2">云服务</h3>
          <p class="text-sm text-muted-foreground">Docker</p>
        </div>
      </div>
    </div>
  </section>

  <!-- 联系我们 -->
  <section class="py-20 px-4 bg-muted/30">
    <div class="container mx-auto max-w-4xl">
      <div class="text-center space-y-8">
        <div class="space-y-4">
          <h2 class="text-3xl md:text-5xl font-bold">联系我们</h2>
          <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
            有任何问题或建议？我们很乐意听到您的声音
          </p>
        </div>

        <div class="grid md:grid-cols-3 gap-8">
          <!-- 邮箱 -->
          <div
            class="text-center p-6 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300 group"
          >
            <div
              class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mx-auto mb-4 group-hover:bg-primary/20 transition-colors"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                x="0px"
                y="0px"
                width="100"
                height="100"
                viewBox="0 0 48 48"
                class="size-6"
              >
                <path
                  fill="#1e88e5"
                  d="M36.5,12h-7.086l3.793-3.793c0.391-0.391,0.391-1.023,0-1.414s-1.023-0.391-1.414,0L26.586,12 h-5.172l-5.207-5.207c-0.391-0.391-1.023-0.391-1.414,0s-0.391,1.023,0,1.414L18.586,12H12.5C9.467,12,7,14.467,7,17.5v15 c0,3.033,2.467,5.5,5.5,5.5h2c0,0.829,0.671,1.5,1.5,1.5s1.5-0.671,1.5-1.5h14c0,0.829,0.671,1.5,1.5,1.5s1.5-0.671,1.5-1.5h2 c3.033,0,5.5-2.467,5.5-5.5v-15C42,14.467,39.533,12,36.5,12z M39,32.5c0,1.378-1.122,2.5-2.5,2.5h-24c-1.378,0-2.5-1.122-2.5-2.5 v-15c0-1.378,1.122-2.5,2.5-2.5h24c1.378,0,2.5,1.122,2.5,2.5V32.5z"
                ></path>
                <rect
                  width="2.75"
                  height="7.075"
                  x="30.625"
                  y="18.463"
                  fill="#1e88e5"
                  transform="rotate(-71.567 32.001 22)"
                ></rect>
                <rect
                  width="7.075"
                  height="2.75"
                  x="14.463"
                  y="20.625"
                  fill="#1e88e5"
                  transform="rotate(-18.432 17.998 21.997)"
                ></rect>
                <path
                  fill="#1e88e5"
                  d="M28.033,27.526c-0.189,0.593-0.644,0.896-1.326,0.896c-0.076-0.013-0.139-0.013-0.24-0.025 c-0.013,0-0.05-0.013-0.063,0c-0.341-0.05-0.745-0.177-1.061-0.467c-0.366-0.265-0.808-0.745-0.947-1.477 c0,0-0.29,1.174-0.896,1.49c-0.076,0.05-0.164,0.114-0.253,0.164l-0.038,0.025c-0.303,0.164-0.682,0.265-1.086,0.278 c-0.568-0.051-0.947-0.328-1.136-0.821l-0.063-0.164l-1.427,0.656l0.05,0.139c0.467,1.124,1.465,1.768,2.74,1.768 c0.922,0,1.667-0.303,2.209-0.909c0.556,0.606,1.288,0.909,2.209,0.909c1.856,0,2.55-1.288,2.765-1.843l0.051-0.126l-1.427-0.657 L28.033,27.526z"
                ></path>
              </svg>
            </div>
            <h3 class="font-semibold mb-2">Bilibili</h3>
            <p class="text-muted-foreground">
              <a
                href="https://space.bilibili.com/4279370"
                target="_blank"
                class="text-primary hover:underline"
              >
                月离离离离
              </a>
            </p>
          </div>

          <!-- GitHub -->
          <div
            class="text-center p-6 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300 group"
          >
            <div
              class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mx-auto mb-4 group-hover:bg-primary/20 transition-colors"
            >
              <svg class="size-6 text-primary" fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                ></path>
              </svg>
            </div>
            <h3 class="font-semibold mb-2">GitHub</h3>
            <p class="text-muted-foreground">
              <a
                href="https://github.com/Yuelioi"
                target="_blank"
                class="text-primary hover:underline"
                >Yuelioi</a
              >
            </p>
          </div>

          <!-- 社区 -->
          <div
            class="text-center p-6 rounded-2xl bg-background border border-border hover:border-primary/50 hover:shadow-lg transition-all duration-300 group"
          >
            <div
              class="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mx-auto mb-4 group-hover:bg-primary/20 transition-colors"
            >
              <!--?xml version="1.0" standalone="no"?-->
              <svg
                class="size-6 icon"
                t="1758142774697"
                viewBox="0 0 1024 1024"
                version="1.1"
                xmlns="http://www.w3.org/2000/svg"
                p-id="1467"
                xmlns:xlink="http://www.w3.org/1999/xlink"
                width="200"
                height="200"
              >
                <path
                  d="M116.435 593.714c-33.54 78.964-38.985 154.297-12.059 168.37 18.572 9.696 47.686-12.382 74.936-52.909 10.791 44.157 37.493 84.137 75.633 116.21-40.004 14.768-66.135 38.886-66.135 66.232 0 44.954 70.709 81.302 157.978 81.302 78.716 0 143.907-29.538 155.94-68.371 3.232-0.049 15.663-0.049 18.796 0 12.083 38.784 77.324 68.371 155.989 68.371 87.267 0 157.978-36.398 157.978-81.302 0-27.297-26.105-51.464-66.135-66.232 38.089-32.123 64.889-72.053 75.631-116.21 27.251 40.527 56.29 62.605 74.887 52.909 26.95-14.073 21.631-89.456-12.032-168.37-26.355-62.058-62.11-107.754-89.457-117.848 0.398-3.929 0.596-7.958 0.596-11.935 0-23.968-6.661-46.146-18.049-64.196 0.199-1.393 0.199-2.834 0.199-4.227 0-11.038-2.636-21.381-7.114-30.332-6.909-161.309-111.93-289.402-281.866-289.402-170.036 0-275.106 128.093-281.943 289.402-4.525 9.001-7.135 19.343-7.135 30.332 0 1.393 0.099 2.835 0.15 4.227-11.288 18.05-17.951 40.178-17.951 64.196 0 3.978 0.15 7.955 0.498 11.935-27.151 10.094-63.028 55.841-89.333 117.848z"
                  fill="currentColor"
                  p-id="1468"
                ></path>
              </svg>
            </div>
            <h3 class="font-semibold mb-2">社区</h3>
            <p class="text-muted-foreground">
              <a
                href="https://qm.qq.com/q/iy4xlZeIc8"
                target="_blank"
                class="text-primary hover:underline"
                >QQ群</a
              >
            </p>
          </div>
        </div>

        <!-- CTA -->
        <div class="pt-8">
          <a
            href="/"
            class="group inline-flex items-center justify-center px-8 py-4 text-lg font-semibold text-primary-foreground bg-primary rounded-full hover:bg-primary/90 transition-all duration-300 hover:scale-105 hover:shadow-lg hover:shadow-primary/25"
          >
            开始使用
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
