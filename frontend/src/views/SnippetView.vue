<template>
  <div class="container mx-auto px-4 py-8 max-w-4xl">
    <PasswordForm
      v-if="passwordRequired"
      :id="id"
      :wrong-password="wrongPassword"
      @submit="onPasswordSubmit"
    />

    <div
      v-else
      class="card bg-card rounded-xl shadow-2xl p-8 flex flex-col gap-6 border border-border backdrop-blur-sm"
      style="box-shadow: var(--shadow-2xl)"
    >
      <!-- 标题和语言标签区域 -->
      <div
        class="flex flex-col sm:flex-row items-start sm:items-center gap-4 pb-6 border-b border-slate-700/50"
      >
        <div class="flex-1">
          <h2 class="text-3xl font-bold bg-gradient-to-r bg-clip-text mb-3">
            {{ snippet?.title || '无标题' }}
          </h2>
        </div>

        <!-- 操作按钮组 -->
        <div class="flex items-center gap-3">
          <button
            @click="shareLink"
            class="group relative overflow-hidden bg-gradient-to-r from-violet-600 to-purple-600 hover:from-violet-500 hover:to-purple-500 text-white px-4 py-2.5 rounded-lg font-medium transition-all duration-300 transform hover:scale-105 hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-purple-500/50"
          >
            <div class="flex items-center gap-2">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="w-4 h-4 transition-transform duration-300 group-hover:rotate-6"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path
                  d="M10.25 3a.75.75 0 0 1 0 1.5h-3.5A2.25 2.25 0 0 0 4.5 6.75v10.5l.012.23A2.25 2.25 0 0 0 6.75 19.5h10.5a2.25 2.25 0 0 0 2.25-2.25v-2a.75.75 0 0 1 1.5 0v2A3.75 3.75 0 0 1 17.25 21H6.75a3.75 3.75 0 0 1-3.745-3.557L3 17.25V6.75A3.75 3.75 0 0 1 6.75 3zm4.687-.932a.75.75 0 0 1 .801.113l7 6a.75.75 0 0 1 .032 1.109l-7 6.75a.75.75 0 0 1-1.27-.54v-2.976c-1.014.064-1.97.273-2.94.769c-1.136.581-2.344 1.581-3.689 3.303l-.271.354a.75.75 0 0 1-1.35-.45c0-2.857.687-5.59 2.168-7.628c1.376-1.893 3.41-3.147 6.082-3.344V2.75l.008-.109a.75.75 0 0 1 .429-.573M16 6.25a.75.75 0 0 1-.75.75c-2.557 0-4.395 1.07-5.618 2.753c-.878 1.208-1.454 2.757-1.717 4.493c1-1.048 1.977-1.785 2.962-2.29C12.36 11.199 13.8 11 15.25 11a.75.75 0 0 1 .75.75v1.984l5.135-4.952L16 4.38z"
                ></path>
              </svg>
              <span>分享</span>
            </div>
            <div
              class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/10 to-white/0 translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-700"
            ></div>
          </button>

          <button
            @click="copyCode"
            class="group relative overflow-hidden bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white px-4 py-2.5 rounded-lg font-medium transition-all duration-300 transform hover:scale-105 hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
          >
            <div class="flex items-center gap-2">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="w-4 h-4 transition-transform duration-300 group-hover:rotate-6"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <path
                  d="M19.4 20H9.6a.6.6 0 0 1-.6-.6V9.6a.6.6 0 0 1 .6-.6h9.8a.6.6 0 0 1 .6.6v9.8a.6.6 0 0 1-.6.6"
                ></path>
                <path
                  d="M15 9V4.6a.6.6 0 0 0-.6-.6H4.6a.6.6 0 0 0-.6.6v9.8a.6.6 0 0 0 .6.6H9"
                ></path>
              </svg>
              <span>复制</span>
            </div>
            <div
              class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/10 to-white/0 translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-700"
            ></div>
          </button>
        </div>
      </div>

      <!-- 代码区域 -->
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <label for="content" class="text-lg font-semibold flex items-center gap-3">
            <div class="p-2 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-lg">
              <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
                ></path>
              </svg>
            </div>
            代码内容
          </label>
          <div class="flex items-center gap-2 text-sm text-gray-400">
            <div class="flex items-center gap-1">
              <div class="w-3 h-3 bg-red-500 rounded-full"></div>
              <div class="w-3 h-3 bg-yellow-500 rounded-full"></div>
              <div class="w-3 h-3 bg-green-500 rounded-full"></div>
            </div>
          </div>
        </div>

        <div class="relative group">
          <CodeViewer v-if="snippet" :content="snippet.content" :language="snippet.language" />

          <div
            class="absolute group-hover:block hidden top-4 right-4 text-xs text-gray-300 font-mono bg-slate-800/80 px-3 py-1 rounded-full"
          >
            {{ snippet?.language || 'JavaScript' }}
          </div>
        </div>
      </div>

      <!-- 作者信息 -->
      <div class="space-y-3">
        <label
          for="author"
          class="text-lg font-semibold text-card-foreground flex items-center gap-3"
          style="font-family: var(--font-sans)"
        >
          <div
            class="p-2 bg-secondary text-secondary-foreground"
            style="border-radius: var(--radius-lg)"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              ></path>
            </svg>
          </div>
          作者
        </label>
        <input
          type="text"
          readonly
          id="author"
          name="author"
          :value="snippet?.author || '佚名'"
          class="w-full h-12 border-2 border-border bg-background px-4 py-3 text-foreground font-medium backdrop-blur-sm transition-all duration-300 hover:border-muted-foreground focus:border-primary focus:bg-muted focus:outline-none focus:ring-0"
          style="border-radius: var(--radius-lg); font-family: var(--font-sans)"
        />
      </div>

      <!-- 描述 -->
      <div class="space-y-3">
        <label
          for="description"
          class="text-lg font-semibold text-card-foreground flex items-center gap-3"
          style="font-family: var(--font-sans)"
        >
          <div class="p-2 bg-accent text-accent-foreground" style="border-radius: var(--radius-lg)">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 12h16M4 18h7"
              ></path>
            </svg>
          </div>
          描述
        </label>
        <textarea
          readonly
          v-model="snippet.description"
          rows="4"
          class="w-full border-2 border-border bg-background px-4 py-3 text-foreground backdrop-blur-sm resize-none transition-all duration-300 hover:border-muted-foreground focus:border-primary focus:bg-muted focus:outline-none focus:ring-0"
          style="border-radius: var(--radius-lg); font-family: var(--font-sans)"
        ></textarea>
      </div>

      <!-- 标签区域 -->
      <div class="space-y-3">
        <label
          for="tags"
          class="text-lg font-semibold text-card-foreground flex items-center gap-3"
          style="font-family: var(--font-sans)"
        >
          <div class="p-2 bg-accent text-accent-foreground" style="border-radius: var(--radius-lg)">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"
              ></path>
            </svg>
          </div>
          标签
        </label>
        <div class="flex flex-wrap gap-3">
          <span
            v-if="snippet.tags && snippet.tags.length === 0"
            class="text-muted-foreground italic py-2"
            >暂无标签</span
          >
          <template v-if="snippet.tags !== ''">
            <span
              v-for="tag in snippet.tags?.split(',')"
              :key="tag"
              class="inline-flex items-center bg-primary text-primary-foreground gap-2 px-3 py-2 text-sm font-semibold rounded-lg shadow-md transition-all duration-300 ease-in-out transform hover:-translate-y-0.5 hover:scale-105 hover:shadow-lg before:content-['🏷️'] before:text-sm"
              >{{ tag }}</span
            >
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import CodeViewer from '@/components/CodeViewer.vue'
import PasswordForm from '@/components/PasswordForm.vue'

import { getSnippet } from '@/api/snippet'
import type { Snippet } from '@/models/snippet'
import { toast } from '@/plugins/toast'

const route = useRoute()
const router = useRouter()

// 获取 URL param
const id = route.params.id as string
const searchParams = new URLSearchParams(window.location.search)
const passwordParam = searchParams.get('password') ?? ''

// 状态
const snippet = ref<Snippet>({
  id: '',
  title: '',
  language: '',
  content: '',
  description: '',
  passwordProtected: false,
  createdAt: '',
  updatedAt: '',
  viewCount: 0,
  visibility: 0,
})
const passwordRequired = ref(true)
const wrongPassword = ref(false)

async function fetchSnippet(password = '') {
  try {
    const response = await getSnippet(id, password)
    snippet.value = response.snippet
    passwordRequired.value = false
  } catch (err: any) {
    console.log(err.response)
    if (err.response?.status === 401) {
      passwordRequired.value = true
      wrongPassword.value = true
    } else if (err.response?.status === 404 || err.response?.status === 500) {
      router.replace('/404')
    } else {
      console.error('获取代码片段失败:', err)
    }
  }
}

// 复制和分享功能
async function copyCode() {
  try {
    await navigator.clipboard.writeText(snippet.value?.content || '')
    toast.success('代码已复制 ✨')
  } catch (err) {
    console.error(err)
    toast.error('复制失败，请手动复制')
  }
}

async function shareLink() {
  try {
    await navigator.clipboard.writeText(window.location.href)
    toast.success('链接已复制 🔗')
  } catch (err) {
    console.error(err)
    toast.error('复制失败，请手动复制链接')
  }
}

onMounted(() => {
  fetchSnippet(passwordParam)
})

// 密码表单提交回调
function onPasswordSubmit(password: string) {
  fetchSnippet(password)
}
</script>
