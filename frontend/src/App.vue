<script setup lang="ts">
import { RouterView } from 'vue-router'
import Header from '@/components/TheHeader.vue'
import Footer from '@/components/TheFooter.vue'
import ToastContainer from '@/plugins/toast/ToastContainer.vue'
</script>

<template>
  <ToastContainer />
  <Header />

  <RouterView />

  <Footer />
</template>
