<template>
  <div class="container mx-auto px-4 py-8 max-w-4xl">
    <!-- Header -->
    <div class="text-center mb-8">
      <h1 class="text-4xl font-bold text-foreground mb-2">分享你的代码</h1>
      <p class="text-muted-foreground">创建一个新的代码片段并与世界分享</p>
    </div>

    <!-- Main Form Card -->
    <div class="bg-card border border-border rounded-lg p-8 shadow-lg">
      <form id="snippetForm" class="space-y-6">
        <!-- Title -->
        <div class="space-y-3">
          <label
            for="title"
            class="text-lg font-semibold text-card-foreground flex items-center gap-3"
            style="font-family: var(--font-sans)"
          >
            <div
              class="p-2 bg-accent text-accent-foreground"
              style="border-radius: var(--radius-lg)"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                ></path>
              </svg>
            </div>
            标题
          </label>

          <input
            v-model="title"
            type="text"
            id="title"
            name="title"
            required
            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="给你的代码片段起个名字..."
          />
        </div>

        <!-- Author -->
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
            v-model="author"
            type="text"
            id="author"
            name="author"
            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="你的名字（可选）"
          />
        </div>

        <!-- Language and Visibility Row -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-3">
            <label
              for="language"
              class="text-sm font-medium text-foreground flex items-center gap-2"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
                ></path>
              </svg>
              编程语言
            </label>
            <div class="relative">
              <select
                v-model="language"
                id="language"
                name="language"
                class="flex h-10 appearance-none w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
              >
                <option value="">选择语言...</option>
                <option value="text">Text</option>
                <option value="javascript">JavaScript</option>
                <option value="typescript">TypeScript</option>
                <option value="json">JSON</option>
                <option value="python">Python</option>
                <option value="java">Java</option>
                <option value="go">Go</option>
                <option value="rust">Rust</option>
                <option value="cpp">C++</option>
                <option value="c">C</option>
                <option value="csharp">C#</option>
                <option value="php">PHP</option>
                <option value="ruby">Ruby</option>
                <option value="swift">Swift</option>
                <option value="kotlin">Kotlin</option>
                <option value="html">HTML</option>
                <option value="css">CSS</option>
                <option value="sql">SQL</option>
                <option value="bash">Bash</option>
                <option value="yaml">YAML</option>
                <option value="markdown">Markdown</option>
              </select>
              <div
                class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M19 9l-7 7-7-7"
                  ></path>
                </svg>
              </div>
            </div>
          </div>

          <!-- Visibility -->
          <div class="space-y-3">
            <label
              for="visibility"
              class="text-sm font-medium text-foreground flex items-center gap-2"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                ></path>
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                ></path>
              </svg>
              可见性
            </label>
            <div class="relative">
              <select
                v-model="visibility"
                id="visibility"
                name="visibility"
                class="flex h-10 appearance-none w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
              >
                <option value="public">公开</option>
                <option value="unlisted">不公开列表</option>
                <option value="private">私有</option>
              </select>
              <div
                class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M19 9l-7 7-7-7"
                  ></path>
                </svg>
              </div>
            </div>
          </div>
        </div>

        <CodeEditor v-model="content" :language="language" />
        <!-- Description -->
        <div class="space-y-3">
          <label
            for="description"
            class="text-lg font-semibold text-card-foreground flex items-center gap-3"
            style="font-family: var(--font-sans)"
          >
            <div
              class="p-2 bg-accent text-accent-foreground"
              style="border-radius: var(--radius-lg)"
            >
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
            v-model="description"
            id="description"
            name="description"
            rows="3"
            class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="描述一下这段代码的功能..."
          ></textarea>
        </div>

        <!-- Tags -->
        <div class="space-y-3">
          <label
            for="tags"
            class="text-lg font-semibold text-card-foreground flex items-center gap-3"
            style="font-family: var(--font-sans)"
          >
            <div
              class="p-2 bg-primary text-primary-foreground"
              style="border-radius: var(--radius-lg)"
            >
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
          <input
            @input="onTagsInput"
            v-model="tagsInput"
            type="text"
            id="tags"
            name="tags"
            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="用逗号分隔标签，如：算法, 数据结构, 实用工具"
          />
          <div class="flex flex-wrap gap-2 mt-2">
            <div
              v-for="tag in tags"
              :key="tag"
              class="inline-flex items-center px-2 py-2 rounded-sm text-xs font-medium bg-primary-500/20 text-primary-300 border border-primary-500/30"
            >
              {{ tag }}
            </div>
          </div>
        </div>

        <!-- Security Options -->
        <div class="rounded-lg border border-border p-4 space-y-4">
          <h3 class="text-lg font-semibold text-foreground mb-3 flex items-center gap-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
              ></path>
            </svg>
            安全选项
          </h3>

          <!-- Password Protection -->
          <div class="flex items-center space-x-3">
            <input
              type="checkbox"
              v-model="passwordProtected"
              id="passwordProtected"
              name="passwordProtected"
              class="h-4 w-4 rounded border-border text-primary focus:ring-2 focus:ring-ring focus:ring-offset-2"
            />
            <label for="passwordProtected" class="text-sm text-foreground">启用密码保护</label>
          </div>

          <!-- Password Input -->
          <div id="passwordField" v-if="passwordProtected" class="space-y-3">
            <input
              v-model="password"
              type="text"
              id="password"
              name="password"
              class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
              placeholder="设置访问密码"
            />
          </div>

          <!-- Expiration -->
          <div class="space-y-3">
            <label
              for="expiresAt"
              class="text-sm font-medium text-foreground flex items-center gap-2"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                ></path>
              </svg>
              过期时间（可选）
            </label>

            <div class="relative w-full">
              <select
                v-model="expiresAt"
                id="expiresAt"
                name="expiresAt"
                class="appearance-none pr-10 flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
              >
                <option value="10min">10分钟</option>
                <option value="60min">1小时</option>
                <option value="1day">1天</option>
                <option value="7days">7天</option>
                <option value="30days">30天</option>
                <option value="0">永久 (注意保护个人隐私)</option>
              </select>
              <div
                class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M19 9l-7 7-7-7"
                  ></path>
                </svg>
              </div>
            </div>
          </div>
        </div>

        <!-- Submit Buttons -->
        <div class="flex flex-col sm:flex-row gap-4">
          <!-- New Format Button -->
          <button
            type="button"
            @click="handleFormat"
            class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 cursor-pointer disabled:pointer-events-none disabled:opacity-50 border border-input bg-background text-foreground hover:bg-muted h-10 px-4 py-2 flex-1 order-last sm:order-first"
          >
            <span class="inline-flex items-center gap-2">
              <svg class="w-4 h-4 flex-none" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M4 6h16M4 10h16M4 14h16M4 18h16"
                ></path>
              </svg>
              <span>格式化代码</span>
            </span>
          </button>

          <!-- Original Submit Button -->
          <button
            type="submit"
            id="submit"
            @click="handleSubmit"
            class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 cursor-pointer disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2 flex-1"
          >
            <span class="flex items-center justify-center gap-2">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                ></path>
              </svg>
              创建代码片段
            </span>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { createSnippet } from '@/api/snippet'
import CodeEditor from '@/components/CodeEditor.vue'
import { toast } from '@/plugins/toast'
import type { CreateSnippetRequest } from '@/models/snippet'

// 导入 Prettier 核心及其所有必要的解析器 (Parser)
import { format as prettierFormat } from 'prettier/standalone'
import * as parserBabel from 'prettier/parser-babel' // JS, JSX, JSON
import * as parserHtml from 'prettier/parser-html'
import * as parserTypescript from 'prettier/parser-typescript'
import * as parserPostcss from 'prettier/parser-postcss' // CSS, Less, SCSS
import * as parserYaml from 'prettier/parser-yaml' // YAML
import * as parserMarkdown from 'prettier/parser-markdown' // Markdown
import * as prettierPluginEstree from 'prettier/plugins/estree'

const visibilityMap: Record<string, number> = {
  public: 0,
  unlisted: 1,
  private: 2,
}

// 过期时间映射表 (秒)
const expiresMap: Record<string, number> = {
  '0': 0, // 永久
  '10min': 10 * 60,
  '60min': 60 * 60,
  '1day': 24 * 60 * 60,
  '7days': 7 * 24 * 60 * 60,
  '30days': 30 * 24 * 60 * 60,
}

// refs
const content = ref('')
const passwordProtected = ref(false)
const password = ref('')
const tagsInput = ref('')
const tags = ref<string[]>([])
const title = ref('')
const language = ref('text')
const description = ref('')
const author = ref('匿名用户')
const visibility = ref('public')
const expiresAt = ref('10min')

/**
 * 将应用中的语言名称映射到 Prettier 内部的 parser 名称
 */
const getPrettierConfig = (lang: string) => {
  const lowerLang = lang.toLowerCase()

  // 默认的 Prettier 格式化选项
  const baseOptions = {
    tabWidth: 2,
    useTabs: false,
    semi: true,
    singleQuote: true,
    printWidth: 100,
    trailingComma: 'es5' as const,
  }

  const languageMap: Record<string, string> = {
    javascript: 'babel',
    typescript: 'typescript',
    json: 'json',
    html: 'html',
    css: 'css',
    less: 'less',
    scss: 'scss',
    markdown: 'markdown', // Updated
    yaml: 'yaml', // Updated
  }

  const parser = languageMap[lowerLang]

  // 传递导入的解析器对象作为插件列表
  const plugins = [
    parserBabel,
    parserHtml,
    parserTypescript,
    parserPostcss,
    parserYaml, // Added
    parserMarkdown, // Added
    prettierPluginEstree,
  ]

  return parser ? { ...baseOptions, parser, plugins } : null
}

// 代码格式化处理函数
async function handleFormat() {
  if (!content.value) {
    toast.error('代码内容不能为空', { title: '系统提示' })
    return
  }

  const options = getPrettierConfig(language.value)

  if (!options) {
    toast.info(`Prettier 暂不支持 ${language.value} 语言的格式化。`, { title: '格式化提示' })
    return
  }

  try {
    // @ts-expect-error https://github.com/prettier/prettier/issues/15473
    const formattedContent = await prettierFormat(content.value, options)

    if (formattedContent !== content.value) {
      content.value = formattedContent.trim()
      toast.success('代码格式化成功！', { title: '系统提示' })
    } else {
      toast.info('代码已是最新格式，无需调整。', { title: '格式化提示' })
    }
  } catch (error: any) {
    console.error('Prettier Formatting error:', error)
    toast.error(`格式化失败，请检查代码语法是否正确。错误信息: ${error.message.split('\n')[0]}`, {
      title: '格式化错误',
    })
  }
}

// 表单提交
async function handleSubmit(e: Event) {
  e.preventDefault()

  if (!content.value) {
    toast.error('代码内容不能为空', { title: '系统提示' })
    return
  }

  const req: CreateSnippetRequest = {
    title: title.value || '未命名片段',
    language: language.value || 'text',
    content: content.value,
    description: description.value || '暂无描述',
    password: passwordProtected.value ? password.value : '',
    expiresInSeconds: expiresMap[expiresAt.value] ?? 0,
    tags: tagsInput.value,
    author: author.value || '匿名用户',
    visibility: visibilityMap[visibility.value] ?? 0,
  }

  try {
    const result = await createSnippet(req)
    const snippet = result.snippet
    if (!snippet) {
      throw new Error('Snippet is null')
    }
    toast.success('创建成功')
    setTimeout(() => {
      const redirectUrl = req.password
        ? `${window.location.origin}/s/${snippet.id}?password=${req.password}`
        : `${window.location.origin}/s/${snippet.id}`
      window.location.href = redirectUrl
    }, 750)
  } catch (error: unknown) {
    toast.error('创建失败')
    console.error('创建失败:', error)
  }
}

// 标签输入监听
function onTagsInput() {
  tags.value = tagsInput.value
    .split(',')
    .map((t) => t.trim())
    .filter((t) => t.length > 0)
}
</script>
