<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, defineEmits, watch } from 'vue'
import { EditorView, basicSetup } from 'codemirror'
import { Compartment } from '@codemirror/state'
import { dracula } from '@uiw/codemirror-theme-dracula'
import { javascript } from '@codemirror/lang-javascript'
import { python } from '@codemirror/lang-python'
import { java } from '@codemirror/lang-java'
import { cpp } from '@codemirror/lang-cpp'
import { php } from '@codemirror/lang-php'
import { html } from '@codemirror/lang-html'
import { css } from '@codemirror/lang-css'
import { sql } from '@codemirror/lang-sql'
import { json } from '@codemirror/lang-json'
import { markdown } from '@codemirror/lang-markdown'
import { rust } from '@codemirror/lang-rust'
import { yaml } from '@codemirror/lang-yaml'
import { go } from '@codemirror/lang-go'

const editorContainer = ref<HTMLDivElement | null>(null)

import type { Extension } from '@codemirror/state'

type LangExtension = (() => Extension) | null

const languageExtensions: Record<string, LangExtension> = {
  javascript,
  typescript: javascript,
  python,
  java,
  cpp,
  rust,
  c: cpp,
  php,
  ruby: cpp,
  swift: cpp,
  kotlin: cpp,
  html,
  css,
  sql,
  json,
  yaml,
  markdown,
  go,
  text: null,
}

const props = defineProps<{ language: string }>()
const editorView = ref<EditorView | null>(null)
const languageConf = new Compartment()
const themeConf = new Compartment()

const emits = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

onMounted(() => {
  if (!editorContainer.value) return

  editorView.value = new EditorView({
    parent: editorContainer.value,
    doc: '',
    extensions: [
      basicSetup,
      languageConf.of([]),
      themeConf.of(dracula),
      EditorView.editorAttributes.of({ class: 'w-full h-96' }),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          const content = update.state.doc.toString()
          emits('update:modelValue', content)
        }
      }),
    ],
  })
})

function updateLanguage(lang: string) {
  const ext = languageExtensions[lang] ?? null
  editorView.value?.dispatch({
    effects: languageConf.reconfigure(ext ? ext() : []),
  })
}

onBeforeUnmount(() => {
  editorView.value?.destroy()
})

watch(
  () => props.language,
  (lang) => {
    updateLanguage(lang)
  },
)
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <label for="editor" class="text-lg font-semibold flex items-center gap-3">
        <div class="p-2 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-lg">
          <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
            />
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

    <!-- 编辑器容器 -->
    <div
      ref="editorContainer"
      id="editor"
      class="flex min-h-[300px] w-full rounded-md border border-input bg-muted px-3 py-2 text-sm ring-offset-background font-mono placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
    ></div>
  </div>
</template>
