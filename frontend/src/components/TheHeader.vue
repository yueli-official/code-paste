<template>
  <header
    class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
  >
    <div class="container mx-auto flex h-16 max-w-screen-2xl items-center justify-between px-4">
      <!-- Logo 区域 -->
      <div class="flex items-center space-x-2">
        <a href="/" class="flex items-center space-x-2 hover:opacity-80 transition-opacity">
          <div class="relative">
            <img
              :src="logo"
              :alt="SITE_TITLE"
              width="80"
              height="30"
              class="shadow-sm ring-1 ring-border/20"
            />
          </div>
        </a>
      </div>

      <!-- 中央导航 -->
      <nav class="hidden md:flex items-center space-x-1">
        <HeaderLink href="/" label="首页" />
        <HeaderLink href="/new" label="新建" />
        <HeaderLink href="/about" label="关于" />
      </nav>

      <!-- 右侧功能区 -->
      <div class="flex items-center space-x-2">
        <ShareLinks />
        <ThemeToggle />

        <!-- 移动端菜单按钮 -->
        <button
          type="button"
          id="mobile-menu-toggle"
          class="md:hidden inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 w-9"
          aria-label="菜单"
          @click="toggleMobileMenu"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 6h16M4 12h16M4 18h16"
            />
          </svg>
        </button>
      </div>
    </div>

    <!-- 移动端菜单面板 -->
    <div
      v-show="mobileMenuOpen"
      id="mobile-menu"
      class="md:hidden border-t border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
    >
      <div class="container px-4 py-4 space-y-3">
        <HeaderLink href="/" label="首页" />
        <HeaderLink href="/new" label="新建" />
        <HeaderLink href="/about" label="关于" />
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import HeaderLink from '@/components/HeaderLink.vue'
import ThemeToggle from '@/components/ThemeToggle.vue'
import ShareLinks from '@/components/ShareLinks.vue'
import logo from '@/assets/logo.png'
import { SITE_TITLE } from '../consts'

// 移动端菜单开关
const mobileMenuOpen = ref(false)
const toggleMobileMenu = () => {
  mobileMenuOpen.value = !mobileMenuOpen.value
}

// 点击外部关闭菜单
function handleClickOutside(e: MouseEvent) {
  const menu = document.getElementById('mobile-menu')
  const toggle = document.getElementById('mobile-menu-toggle')
  if (!menu?.contains(e.target as Node) && !toggle?.contains(e.target as Node)) {
    mobileMenuOpen.value = false
  }
}

// 滚动阴影效果
function initScrollEffect() {
  const header = document.querySelector('header')
  const onScroll = () => {
    if (window.scrollY > 100) {
      header?.classList.add('shadow-sm')
    } else {
      header?.classList.remove('shadow-sm')
    }
  }
  window.addEventListener('scroll', onScroll)
  return () => window.removeEventListener('scroll', onScroll)
}

let cleanupScroll: (() => void) | undefined

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  cleanupScroll = initScrollEffect()
})
onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
  cleanupScroll?.()
})
</script>

<style scoped>
/* 自定义滚动条（可选） */
::-webkit-scrollbar {
  width: 6px;
}
::-webkit-scrollbar-track {
  background: hsl(var(--background));
}
::-webkit-scrollbar-thumb {
  background: hsl(var(--border));
  border-radius: 3px;
}
::-webkit-scrollbar-thumb:hover {
  background: hsl(var(--muted-foreground));
}

/* 导航项目活跃状态 */
.nav-item[aria-current='page'] {
  color: hsl(var(--foreground));
}
.nav-item[aria-current='page'] span {
  opacity: 1;
}

/* 平滑滚动 */
html {
  scroll-behavior: smooth;
}
@media (prefers-reduced-motion: no-preference) {
  * {
    scroll-behavior: smooth;
  }
}
</style>
