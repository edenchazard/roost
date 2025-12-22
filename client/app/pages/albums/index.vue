<template>
  <div class="p-4">
    <h1 class="text-xl mb-4">Albums</h1>
    <ol class="grid gap-8 grid-cols-[repeat(auto-fill,minmax(15rem,17rem))]">
      <li
        v-for="album in data"
        :key="album.id"
        class="h-60"
      >
        <Album
          :album="album"
          as="a"
          :href="`/albums/${album.id}`"
        />
      </li>
    </ol>
  </div>
</template>

<script setup lang="ts">
import Album from "@/components/Album.vue";
import type { Roost } from "~/types/roost";

const { data } = useApi<Roost.Album[]>("albums");
</script>
