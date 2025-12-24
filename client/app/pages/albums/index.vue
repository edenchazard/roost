<template>
  <div class="p-4">
    <Teleport
      to="#splash"
      defer
    >
      <Transition
        class="transition-opacity duration-1000"
        appear-from-class="opacity-0"
        enter-to-class="opacity-20"
        appear
        @after-appear="backgroundSplash?.classList.add('opacity-20')"
      >
        <div
          class="size-full bg-repeat"
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
      class="col-span-full p-4"
      :style="{
        backgroundColor: backgroundStyle,
      }"
    >
      <div
        class="text-xl bg-linear-to-r from-black/20 to-black/0 p-4 rounded-lg font-semibold"
      >
        <h2 class="inline-block">
          {{ openedAlbum.title }}
        </h2>
        <span class="text-sm"> &bull; 2022 </span>
      </div>
      <div>
        <h3 class="text-lg">{{ openedAlbum.artist }}</h3>
        <ol class="list-decimal list-inside">
          <li v-for="track in tracks">{{ track.title }}</li>
        </ol>
      </div>
    </div>

    <h1 class="text-xl mb-4">Albums</h1>
    <ol
      class="grid grid-cols-[repeat(auto-fill,15rem)]"
      ref="albumGrid"
    >
      <li
        v-for="album in data"
        :key="album.id"
        class="size-60 px-2 pt-2"
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
