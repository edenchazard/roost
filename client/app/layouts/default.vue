<template>
  <div class="h-dvh bg-black text-white flex flex-col">
    <header class="border-b border-stone-800">
      <div class="text-white p-4">logo</div>
    </header>

    <div class="flex-1 flex gap-8">
      <div class="p-4 text-sm">
        <nav>
          <SidebarMenuItem
            icon="streamline-pixel:music-album-cd-disk-playlist"
            label="Albums"
          />
        </nav>
      </div>
      <main class="flex-1 h-full bg-stone-950 rounded-tl-lg"><slot /></main>
    </div>
    <Player />
  </div>
</template>

<script setup lang="ts">
import SidebarMenuItem from "@/components/SidebarMenuItem.vue";
</script>
