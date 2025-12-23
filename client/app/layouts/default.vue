<template>
  <div
    class="h-dvh overflow-auto text-white relative bg-black gap-x-8 grid grid-cols-[auto_1fr] grid-rows-[auto_1fr_auto]"
  >
    <div
      id="splash"
      class="fixed -inset-8 pointer-events-none blur-lg"
    ></div>

    <header class="col-span-full sticky z-90 top-0 backdrop-blur-xs">
      <div class="text-white p-4 bg-black/20">logo</div>
    </header>

    <div class="p-4 text-sm z-1 w-44 mt-4">
      <nav class="flex flex-col gap-4 sticky top-18">
        <SidebarMenuItem
          icon="streamline-pixel:music-album-cd-disk-playlist"
          label="Albums"
        />
        <span
          class="h-px block bg-linear-to-r from-transparent via-white/50 to-transparent"
        />
        <SidebarMenuItem
          icon="streamline-pixel:music-album-cd-disk-playlist"
          label="Albums"
        />
        <span
          class="h-px block bg-linear-to-r from-transparent via-white/50 to-transparent"
        />
        <SidebarMenuItem
          icon="streamline-pixel:music-album-cd-disk-playlist"
          label="Albums"
        />
      </nav>
    </div>

    <main class="bg-black/10 z-1">
      <slot />
    </main>

    <KeepAlive>
      <Player />
    </KeepAlive>
  </div>
</template>

<script setup lang="ts">
import SidebarMenuItem from "@/components/SidebarMenuItem.vue";
</script>
