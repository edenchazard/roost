<template>
  <div class="py-4">
    <Teleport
      to="#splash"
      defer
    >
      <Transition
        class="transition-colors"
        appear-from-class="opacity-0"
        enter-to-class="opacity-20"
        @after-enter="backgroundSplash?.classList.add('opacity-20')"
      >
        <div
          class="size-full opacity-50"
          ref="backgroundSplash"
          :style="{
            backgroundColor: backgroundStyle,
          }"
        />
      </Transition>
    </Teleport>

    <div
      v-if="openedAlbum"
      ref="openAlbumListingPanel"
      class="col-span-full -top-4 pt-4 relative justify-self-stretch"
      :style="{
        backgroundColor: backgroundStyle,
      }"
    >
      <div class="px-2 space-y-4">
        <div
          class="bg-linear-to-r from-black/20 to-black/0 p-4 rounded-lg font-semibold space-y-2"
        >
          <h2 class="text-xl inline-block">
            {{ openedAlbum.title }}
          </h2>
          <span class="text-sm"> &bull; 2022 </span>
          <h3 class="text-sm">{{ openedAlbum.artist }}</h3>
        </div>

        <ol class="list-none columns-[20rem_auto] space-y-2 text-sm">
          <li
            v-for="track in tracks"
            class="bg-black/25 rounded-lg py-2 px-4 flex items-center gap-2"
          >
            <span
              v-if="track.trackNumber"
              class="opacity-50 w-2.5"
              >{{ track.trackNumber }}</span
            >
            <span class="flex-1">{{ track.title }}</span>
            <span class="opacity-50 w-12 text-right"> 3:21 </span>
          </li>
        </ol>
      </div>
      <img
        :src="openedAlbum.pictureUrl"
        class="h-4 w-full object-cover mask-t-to-100%"
      />
    </div>

    <h1 class="text-xl mb-4 px-2">Albums</h1>
    <ol
      class="grid grid-cols-[repeat(auto-fill,minmax(15rem,1fr))] gap-y-4 w-full"
      ref="albumGrid"
    >
      <li
        v-for="album in data"
        :key="album.id"
        class="size-60"
        :class="{
          'px-2 pt-2': openedAlbum?.id === album.id,
        }"
        :style="{
          backgroundColor:
            openedAlbum?.id === album.id ? backgroundStyle : 'transparent',
        }"
        :data-album-id="album.id"
      >
        <Album
          :album="album"
          as="button"
          :href="`/albums/${album.id}`"
          @click="handleToggleAlbum(album)"
        />
      </li>
    </ol>
  </div>
</template>

<script setup lang="ts">
import ColorThief from "colorthief";
import Album from "@/components/Album.vue";
import type { Roost } from "~/types/roost";

const albumGrid = useTemplateRef("albumGrid");
const backgroundSplash = useTemplateRef("backgroundSplash");
const openAlbumListingPanel = useTemplateRef("openAlbumListingPanel");

const openedAlbum = ref<Roost.Album>();
const openedAlbumColour = ref<[number, number, number] | "transparent">(
  "transparent"
);

useResizeObserver(albumGrid, recomputeAlbumListingPanel);

const { data } = useApi<Roost.Album[]>("albums");

const { data: tracks, execute: fetchTracks } = useApi<Roost.Track[]>(
  computed(() => {
    return openedAlbum.value ? `albums/${openedAlbum.value.id}/tracks` : "";
  }),
  {
    immediate: false,
  }
);

const backgroundStyle = computed<string>(() => {
  return Array.isArray(openedAlbumColour.value)
    ? "rgb(" + openedAlbumColour.value.join(",") + ")"
    : "transparent";
});

const albumElement = computed(() => {
  if (!openedAlbum.value) return null;

  return albumGrid.value?.querySelector(
    `[data-album-id='${openedAlbum.value?.id}']`
  );
});

watch(openedAlbum, async (newAlbum) => {
  openedAlbumColour.value = "transparent";
  if (!newAlbum?.pictureUrl) {
    return;
  }

  const img = document.createElement("img");
  img.crossOrigin = "Anonymous";

  img.addEventListener("load", async () => {
    const [colour] = await Promise.all([
      new ColorThief().getColor(img),
      fetchTracks(),
    ]);
    recomputeAlbumListingPanel();
    openedAlbumColour.value = colour;
  });

  img.src = newAlbum.pictureUrl;
});

async function handleToggleAlbum(album?: Roost.Album) {
  if (openedAlbum.value?.id === album?.id) {
    openedAlbum.value = undefined;
    return;
  }

  openedAlbum.value = album;
}

async function recomputeAlbumListingPanel() {
  await nextTick();

  if (
    !openedAlbum.value ||
    !albumElement.value ||
    !albumGrid.value ||
    !openAlbumListingPanel.value
  ) {
    return;
  }

  if (openAlbumListingPanel.value.parentElement === albumGrid.value) {
    albumGrid.value.removeChild(openAlbumListingPanel.value);
  }

  const albumsPerRow = Math.floor(
    albumGrid.value.clientWidth / albumElement.value.clientWidth
  );

  if (albumsPerRow <= 0) {
    albumGrid.value.appendChild(openAlbumListingPanel.value);
    return;
  }

  const albumItems = Array.from(albumGrid.value.querySelectorAll("li"));
  const albumIndex = albumItems.indexOf(albumElement.value);
  if (albumIndex === -1) {
    albumGrid.value.appendChild(openAlbumListingPanel.value);
    return;
  }

  const remainder = albumIndex % albumsPerRow;
  const offset = remainder === 0 ? albumsPerRow : albumsPerRow - remainder;
  const dropAfterIndex = albumIndex + offset;

  if (dropAfterIndex >= albumItems.length) {
    albumGrid.value.appendChild(openAlbumListingPanel.value);
    return;
  }

  albumGrid.value.insertBefore(
    openAlbumListingPanel.value,
    albumItems[dropAfterIndex]
  );
}
</script>
