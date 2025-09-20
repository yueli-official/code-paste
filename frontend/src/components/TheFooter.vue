<template>
  <footer
    class="relative border-t bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
  >
    <!-- Background decoration -->
    <div
      class="absolute inset-0 bg-gradient-to-r from-transparent via-primary/5 to-transparent"
    ></div>

    <div class="container relative mx-auto px-4 py-12">
      <!-- Main footer content -->
      <div class="flex flex-col items-center space-y-6">
        <!-- Brand section -->
        <div class="text-center space-y-2">
          <h3
            class="text-2xl font-bold bg-gradient-to-r from-primary to-primary/70 bg-clip-text text-transparent"
          >
            月离万事屋
          </h3>
          <p class="text-sm text-muted-foreground max-w-md">
            创造美好的数字体验，用心雕琢每一个细节
          </p>
        </div>

        <!-- Divider -->
        <div class="w-full max-w-xs border-t border-border/50"></div>

        <!-- Social links -->
        <div class="flex items-center space-x-6">
          <a
            href="https://github.com/yuelioi"
            target="_blank"
            rel="noopener noreferrer"
            class="group relative p-3 rounded-full border border-border/50 bg-background/50 hover:bg-primary/10 hover:border-primary/50 transition-all duration-300 hover:scale-110"
            aria-label="访问 Yueli 的 GitHub"
          >
            <svg
              viewBox="0 0 16 16"
              aria-hidden="true"
              class="w-5 h-5 text-muted-foreground group-hover:text-primary transition-colors duration-300"
            >
              <path
                fill="currentColor"
                d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"
              ></path>
            </svg>
            <!-- Hover effect -->
            <div
              class="absolute inset-0 rounded-full bg-primary/20 scale-0 group-hover:scale-100 transition-transform duration-300 -z-10"
            ></div>
          </a>

          <a
            href="https://space.bilibili.com/4279370"
            target="_blank"
            rel="noopener noreferrer"
            class="group relative p-3 rounded-full border border-border/50 bg-background/50 hover:bg-primary/10 hover:border-primary/50 transition-all duration-300 hover:scale-110"
            aria-label="Twitter"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" width="24px" height="24px">
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
            <div
              class="absolute inset-0 rounded-full bg-primary/20 scale-0 group-hover:scale-100 transition-transform duration-300 -z-10"
            ></div>
          </a>
        </div>

        <!-- Copyright -->
        <div class="text-center space-y-2">
          <p class="text-sm text-muted-foreground">
            &copy; {{ new Date().getFullYear() }} 月离万事屋. 保留所有权利
          </p>
        </div>
      </div>
    </div>

    <!-- Bottom accent line -->
    <div class="h-px bg-gradient-to-r from-transparent via-primary/20 to-transparent"></div>
  </footer>
</template>
