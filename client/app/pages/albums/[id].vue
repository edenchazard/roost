<template>
  <div
    v-if="album"
    class="relative size-full"
  >
    <Transition
      class="transition-opacity duration-1000"
      appear-from-class="opacity-0"
      enter-to-class="opacity-20"
      appear
      @after-appear="backgroundSplash?.classList.add('opacity-20')"
    >
      <div
        ref="backgroundSplash"
        class="absolute bg-cover inset-0 select-none"
        :style="{
          backgroundImage: `url(${album.pictureUrl})`,
        }"
      />
    </Transition>
    <div
      class="grid grid-cols-[auto_1fr] grid-rows-[auto_1fr] gap-4 h-full backdrop-blur-lg p-4"
    >
      <div class="relative rounded-md overflow-hidden shrink-0">
        <img
          :src="album?.pictureUrl"
          alt="Album Cover Art"
          class="size-64 object-cover"
        />
        <div class="h-16 overflow-hidden">
          <div
            aria-hidden="true"
            class="w-64 h-64 bg-cover -scale-y-100 mask-t-from-0% mask-t-to-25%"
            :style="{
              backgroundImage: `url(${album?.pictureUrl})`,
            }"
          />
        </div>
      </div>
      <div class="p-8">
        <h1 class="text-xl mb-4">{{ album.title }}</h1>
        <h2 class="text-base">{{ album.artist }}</h2>
      </div>

      <div
        class="col-span-full bg-linear-to-b from-black/60 to-black/10 p-4 rounded-md"
      >
        <h2 class="text-xl sr-only">Tracks</h2>
        <table class="border-separate w-full font-bold border-spacing-y-1.5">
          <thead>
            <tr class="uppercase text-xs">
              <th><span class="sr-only">Play</span></th>
              <th class="p-2 text-center">#</th>
              <th class="text-left p-2">Title</th>
              <th class="text-left p-2">Duration</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="track in tracks"
              :key="track.id"
              class="bg-stone-950/50 text-sm *:first:rounded-l-lg *:last:rounded-r-lg"
            >
              <td class="p-2 text-center">
                <button>
                  <Icon
                    name="mdi:play-circle-outline"
                    size="1.5em"
                    @click="
                      usePlayerStore.current.value = track;
                      usePlayerStore.togglePlay();
                    "
                  />
                </button>
              </td>
              <td class="p-2 text-center">
                {{ track.trackNumber }}
              </td>
              <td class="p-2">{{ track.title }}</td>
              <td class="p-2">444</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Roost } from "~/types/roost";
import usePlayerStore from "~/stores/usePlayerStore";

const backgroundSplash = useTemplateRef("backgroundSplash");

const { data: album } = useApi<Roost.Album>(`albums/${useRoute().params.id}`);
const { data: tracks } = useApi<Roost.Track[]>(
  `albums/${useRoute().params.id}/tracks`
);
</script>
