<template>
  <div ref="editorContainer"
    class="min-h-[350px] max-h-[70vh]  w-full overflow-x-hidden overflow-y-auto rounded-md border-2 border-slate-700/50 bg-slate-900/80 backdrop-blur-sm shadow-inner transition-all duration-300 hover:border-slate-600/50 focus-within:border-indigo-500/50 focus-within:shadow-lg focus-within:shadow-indigo-500/10">
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
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

import type { Extension } from '@codemirror/state'

type LangExtension = (() => Extension) | null

const props = defineProps<{
  content: string
  language: string
}>()

const editorContainer = ref<HTMLDivElement | null>(null)
const languageConf = new Compartment()
let view: EditorView

const languageExtensions: Record<string, LangExtension> = {
  javascript,
  typescript: javascript,
  python,
  java,
  cpp,
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
  rust,
  go: cpp,
  shell: cpp,
  bash: cpp,
}

onMounted(() => {
  const code = props?.content || ''
  const language = props?.language || 'javascript'

  view = new EditorView({
    parent: editorContainer.value!,
    doc: code,
    extensions: [
      basicSetup,
      languageConf.of([]),
      EditorView.theme({
        '&': { fontSize: '14px' },
        '.cm-content': { padding: '20px', minHeight: '300px', fontFamily: 'var(--font-mono)' },
        '.cm-focused': { outline: 'none' },
        '.cm-editor': { borderRadius: '12px' },
        '.cm-scroller': { fontFamily: 'var(--font-mono)' },
      }),
      EditorView.editorAttributes.of({ class: 'w-full' }),
      dracula,
    ],
  })

  const ext = languageExtensions[language.toLowerCase()] ?? null
  view.dispatch({ effects: languageConf.reconfigure(ext ? ext() : []) })
})
</script>
