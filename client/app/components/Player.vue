<template>
  <div
    class="h-24 backdrop-blur-3xl md:mx-auto z-60 shadow-2xl md:w-xl px-4 flex flex-col md:rounded-lg overflow-hidden p-2 bg-linear-to-b from-black/20 to-black/60"
  >
    <div class="flex items-center">
      <img
        src="http://localhost:3001/assets/album-art/luna_夏の夜明けを待つ僕ら.jpg"
        class="size-12 border border-white/20 rounded-md inline-block mr-4 float-left"
      />
      <div class="flex-1 flex flex-col gap-2">
        <div class="text-xs text-center">
          <span
            v-if="
              usePlayerStore.current.value?.artist &&
              usePlayerStore.current.value.title
            "
          >
            {{ usePlayerStore.current.value.artist }} &mdash;
            {{ usePlayerStore.current.value.title }}
          </span>
        </div>
        <div class="flex justify-center gap-8">
          <button aria-label="Previous">
            <Icon
              name="mdi:skip-previous"
              size="2em"
            />
          </button>
          <button
            @click="usePlayerStore.togglePlay"
            :aria-pressed="usePlayerStore.isPlaying.value"
            aria-label="Play/Pause"
          >
            <Icon
              v-if="!usePlayerStore.isPlaying.value"
              size="2em"
              name="mdi:play"
            />
            <Icon
              v-else
              size="2em"
              name="mdi:pause"
            />
          </button>
          <button aria-label="Next">
            <Icon
              name="mdi:skip-next"
              size="2em"
            />
          </button>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-[auto_1fr_auto] items-center gap-2 my-2 text-xs">
      <span class="time">
        {{ formattedTime(usePlayerStore.currentTime.value) }}
      </span>
      <input
        type="range"
        min="0"
        :max="usePlayerStore.duration.value ?? 0"
        class="w-full"
        step="0.1"
        v-model.number="usePlayerStore.seekPos.value"
        @input="usePlayerStore.handleOnSeekInput"
        @change="usePlayerStore.handleOnSeekChange"
      />
      <span class="time">
        {{ formattedTime(usePlayerStore.duration.value) }}
      </span>
    </div>

    <audio
      :ref="usePlayerStore.audioElementRefId"
      :src="usePlayerStore.current.value?.audioUrl"
      @play="usePlayerStore.handleOnPlay"
      @pause="usePlayerStore.handleOnPause"
      @timeupdate="usePlayerStore.handleOnTime"
      @loadedmetadata="usePlayerStore.handleOnLoaded"
      @ended="usePlayerStore.handleOnEnded"
    />
  </div>
</template>

<script lang="ts" setup>
import usePlayerStore from "~/stores/usePlayerStore";

function formattedTime(t: number) {
  if (!t || isNaN(t)) return "0:00";

  const mm = Math.floor(t / 60);
  const ss = Math.floor(t % 60)
    .toString()
    .padStart(2, "0");

  return `${mm}:${ss}`;
}

const audioElement = useTemplateRef<HTMLAudioElement>(
  usePlayerStore.audioElementRefId
);

onMounted(() => {
  usePlayerStore.setAudioElement(audioElement.value);
});
</script>
