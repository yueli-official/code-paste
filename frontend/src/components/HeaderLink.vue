<template>
  <RouterLink
    :to="props.href"
    class="nav-item relative px-4 py-2 text-sm text-muted-foreground hover:text-foreground transition-colors duration-200 group"
    :class="[isActive ? 'text-primary font-bold scale-105' : 'text-foreground font-normal']"
  >
    {{ props.label }}
    <span
      class="absolute inset-x-0 -bottom-px h-px bg-gradient-to-r from-transparent via-blue-500 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"
    />
  </RouterLink>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed } from 'vue'

interface Props {
  href: string
  label: string
}

const props = defineProps<Props>()
const route = useRoute()

// 判断当前路由是否 active
const isActive = computed(() => {
  const pathname = route.path
  const subpath = pathname.match(/[^/]+/g)
  return props.href === pathname || props.href === '/' + (subpath?.[0] || '')
})
</script>
