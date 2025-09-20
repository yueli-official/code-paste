<script setup lang="ts">
interface Props {
  id: string
  wrongPassword?: boolean
}

const props = defineProps<Props>()
</script>

<template>
  <form
    method="GET"
    :action="`/s/${props.id}`"
    class="max-w-md mx-auto p-6 rounded-xl border shadow-lg"
    :style="{
      background: 'var(--color-card)',
      borderColor: 'var(--color-border)',
      color: 'var(--color-card-foreground)',
    }"
  >
    <h2 class="text-xl font-semibold mb-4" :style="{ color: 'var(--color-foreground)' }">
      请输入密码查看代码
    </h2>

    <div class="mb-4">
      <label
        for="password"
        class="block mb-2 text-sm font-medium"
        :style="{ color: 'var(--color-muted-foreground)' }"
      >
        密码
      </label>
      <input
        id="password"
        type="password"
        name="password"
        required
        placeholder="请输入密码"
        class="w-full px-4 py-2 rounded-lg transition focus:outline-none"
        :style="{
          background: 'var(--color-input)',
          color: 'var(--color-foreground)',
          border: '1px solid var(--color-border)',
          boxShadow: '0 0 0 2px var(--color-ring)',
        }"
      />
      <p
        v-if="props.wrongPassword"
        class="text-sm mt-2"
        :style="{ color: 'var(--color-destructive)' }"
      >
        密码错误，请重试。
      </p>
    </div>

    <button
      type="submit"
      class="w-full py-2 px-4 font-semibold rounded-lg transition"
      :style="{
        background: 'var(--color-primary)',
        color: 'var(--color-primary-foreground)',
        boxShadow: 'var(--shadow-sm)',
      }"
    >
      提交
    </button>
  </form>
</template>
