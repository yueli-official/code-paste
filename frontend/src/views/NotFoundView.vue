<template>
  <main class="min-h-screen bg-slate-900 text-white overflow-hidden relative">
    <!-- 动态背景网格 -->
    <div
      class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.05)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.1)_1px,transparent_1px)] bg-[size:75px_75px] [mask-image:radial-gradient(ellipse_at_center,transparent_20%,black)]"
    ></div>

    <!-- 浮动元素 -->
    <div class="absolute inset-0 overflow-hidden pointer-events-none">
      <div class="absolute top-1/4 left-1/4 size-4 bg-blue-500 rounded-full animate-ping"></div>
      <div class="absolute top-3/4 right-1/4 size-2 bg-purple-500 rounded-full animate-pulse"></div>
      <div class="absolute top-1/2 left-3/4 size-4 bg-green-500 rounded-full animate-bounce"></div>
    </div>

    <div class="relative z-10 flex items-center justify-center min-h-screen p-8">
      <div class="max-w-4xl mx-auto text-center">
        <!-- 主要404内容 -->
        <div class="space-y-8">
          <div class="relative">
            <div class="text-[clamp(6rem,20vw,12rem)] font-black leading-none">
              <span
                class="bg-gradient-to-r from-white via-gray-300 to-gray-500 bg-clip-text text-transparent"
              >
                4
              </span>
              <span class="relative inline-block mx-4">
                <span
                  class="bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent animate-pulse"
                >
                  0
                </span>
                <div
                  class="absolute inset-0 bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent blur-sm opacity-50"
                >
                  0
                </div>
              </span>
              <span
                class="bg-gradient-to-r from-gray-500 via-gray-300 to-white bg-clip-text text-transparent"
              >
                4
              </span>
            </div>
          </div>

          <!-- 错误描述 -->
          <div class="space-y-4 max-w-md mx-auto">
            <h1 class="text-2xl font-light text-gray-300">页面不存在</h1>
            <p class="text-gray-500 leading-relaxed">您访问的代码片段可能已被移动或删除</p>
          </div>

          <!-- 搜索框 -->
          <div class="max-w-md mx-auto">
            <div class="relative group">
              <input
                type="text"
                placeholder="搜索代码片段..."
                class="w-full px-6 py-4 bg-white/5 border border-white/10 rounded-2xl text-white placeholder-gray-400 backdrop-blur-sm focus:outline-none focus:border-blue-500/50 focus:bg-white/10 transition-all duration-300"
              />
              <button
                class="absolute right-2 top-1/2 -translate-y-1/2 p-2 text-gray-400 hover:text-white transition-colors"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                  ></path>
                </svg>
              </button>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="flex flex-col sm:flex-row gap-4 justify-center items-center pt-8">
            <a
              href="/"
              class="group relative px-8 py-3 bg-white text-black rounded-full font-medium hover:bg-gray-100 transition-all duration-300 transform hover:scale-105"
            >
              <span class="flex items-center">
                <svg
                  class="w-4 h-4 mr-2 group-hover:-translate-x-1 transition-transform"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
                  ></path>
                </svg>
                返回首页
              </span>
            </a>

            <button
              onclick="history.back()"
              class="group px-8 py-3 border border-white/20 text-white rounded-full font-medium hover:border-white/40 hover:bg-white/5 transition-all duration-300"
            >
              <span class="flex items-center">
                <svg
                  class="w-4 h-4 mr-2 group-hover:-translate-x-1 transition-transform"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M10 19l-7-7m0 0l7-7m-7 7h18"
                  ></path>
                </svg>
                返回上页
              </span>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部光效 -->
    <div
      class="absolute bottom-0 left-1/2 -translate-x-1/2 w-96 h-96 bg-gradient-to-t from-primary/20 via-purple-500/10 to-transparent rounded-full blur-3xl"
    ></div>
  </main>
</template>
