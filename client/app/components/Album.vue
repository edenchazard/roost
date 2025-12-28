<template>
  <component
    :is="as"
    class="group size-full relative inline-flex overflow-hidden flex-col"
  >
    <div
      class="size-full flex items-center justify-center group-hover:scale-110 transition-transform duration-300 self-center"
    >
      <img
        v-if="album.pictureUrl"
        class="object-contain size-full"
        :src="asset(album.pictureUrl)"
      />
      <Icon
        v-else
        name="mage:compact-disk-fill"
        size="8rem"
      />
    </div>
    <div
      class="absolute bottom-0 w-full bg-linear-to-b from-black/70 to-black/50 text-white p-2"
    >
      <h2 class="text-base line-clamp-1 group-hover:line-clamp-none">
        {{ album.title }}
      </h2>
      <h3 class="text-xs line-clamp-1 group-hover:line-clamp-none">
        {{ album.artist }}
      </h3>
    </div>
  </component>
</template>

<script setup lang="ts">
import type { Roost } from "~/types/roost";

withDefaults(
  defineProps<{
    album: Roost.Album;
    as?: string;
  }>(),
  {
    as: "article",
  }
);
</script>
