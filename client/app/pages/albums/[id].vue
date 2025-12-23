<template>
  <div v-if="album">
    <Teleport to="#splash">
      <Transition
        class="transition-opacity duration-1000"
        appear-from-class="opacity-0"
        enter-to-class="opacity-20"
        appear
        @after-appear="backgroundSplash?.classList.add('opacity-20')"
      >
        <div
          ref="backgroundSplash"
          class="absolute bg-cover size-full"
          :style="{
            backgroundImage: `url(${album.pictureUrl})`,
          }"
        />
      </Transition>
    </Teleport>
    <div class="grid grid-cols-[auto_1fr] grid-rows-[auto_auto] gap-4 py-4">
      <div class="relative rounded-md overflow-hidden shrink-0 pl-8">
        <img
          :src="album?.pictureUrl"
          alt="Album Cover Art"
          class="size-64 object-cover"
        />
        <div class="h-16 overflow-hidden">
          <div
            aria-hidden="true"
            class="size-64 bg-cover -scale-y-100 mask-t-from-0% mask-t-to-25%"
            :style="{
              backgroundImage: `url(${album?.pictureUrl})`,
            }"
          />
        </div>
      </div>

      <div class="p-8">
        <div
          class="text-xl mb-4 bg-linear-to-r from-black/20 to-black/0 p-4 rounded-lg font-semibold"
        >
          <h1 class="inline-block">
            {{ album.title }}
          </h1>
          <span class="text-sm"> &bull; 2022 </span>
        </div>
        <h2 class="text-base ml-4">{{ album.artist }}</h2>
      </div>

      <section
        class="col-span-full bg-linear-to-b from-black/20 to-black/10 py-4 px-8"
      >
        <h2 class="text-xl">Tracks</h2>
        <table class="border-separate w-full font-bold border-spacing-y-1.5">
          <thead>
            <tr class="uppercase text-xs *:not-[.edge]:bg-black/50">
              <th
                class="edge bg-linear-to-r from-20% from-black/10 to-black/50"
              >
                <span class="sr-only">Play</span>
              </th>
              <th class="p-2 text-center">#</th>
              <th class="text-left p-2">Title</th>
              <th
                class="edge bg-linear-to-r to-80% from-black/50 to-black/10 text-center p-2"
              >
                Duration
              </th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="track in tracks"
              :key="track.id"
              class="bg-black/15 text-sm *:first:rounded-l-lg *:last:rounded-r-lg"
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
      </section>

      <section
        class="col-span-full bg-linear-to-b from-black/20 to-black/10 py-4 px-8"
      >
        <h2 class="text-xl">By This Artist</h2>
        <div class="flex gap-4 overflow-x-hidden py-4">
          <ol class="flex gap-4">
            <li
              v-for="n in 5"
              :key="n"
              class="size-48 shadow-xl"
            >
              <Album
                :album="{
                  id: n,
                  title: `Album Title ${n}`,
                  artist: album.artist,
                  pictureUrl: album.pictureUrl,
                }"
              />
            </li>
          </ol>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Roost } from "~/types/roost";
import usePlayerStore from "~/stores/usePlayerStore";
import Album from "~/components/Album.vue";

const backgroundSplash = useTemplateRef("backgroundSplash");

const { data: album } = useApi<Roost.Album>(`albums/${useRoute().params.id}`);
const { data: tracks } = useApi<Roost.Track[]>(
  `albums/${useRoute().params.id}/tracks`
);
</script>
