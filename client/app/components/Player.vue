<template>
  <div
    class="shadow-2xl absolute left-1/2 w-xl px-4 -translate-x-1/2 bottom-2 flex flex-col rounded-lg overflow-hidden p-2 bg-linear-to-b from-black/20 to-black/60"
  >
    <div class="flex items-center">
      <img
        src="http://localhost:3001/assets/album-art/Love_Wing_Bell.webp"
        class="size-12 border border-white/20 rounded-md inline-block mr-4 float-left"
      />
      <div class="flex-1 flex flex-col gap-2">
        <div class="text-xs text-center">Artist Name &mdash; Song Title</div>
        <div class="flex justify-center gap-8">
          <button
            @click="handlePrevious"
            aria-label="Previous"
          >
            <Icon
              name="mdi:skip-previous"
              size="2em"
            />
          </button>
          <button
            @click="togglePlay"
            :aria-pressed="isPlaying"
            aria-label="Play/Pause"
          >
            <Icon
              v-if="!isPlaying"
              size="2em"
              name="mdi:play"
            />
            <Icon
              v-else
              size="2em"
              name="mdi:pause"
            />
          </button>
          <button
            @click="handleNext"
            aria-label="Next"
          >
            <Icon
              name="mdi:skip-next"
              size="2em"
            />
          </button>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-[auto_1fr_auto] items-center gap-2 my-2">
      <span class="time">{{ formattedTime(currentTime) }}</span>
      <input
        type="range"
        min="0"
        :max="duration || 0"
        class="w-full"
        step="0.1"
        v-model.number="seekPos"
        @input="handleOnSeekInput"
        @change="handleOnSeekChange"
      />
      <span class="time">{{ formattedTime(duration) }}</span>
    </div>

    <audio
      ref="audio"
      :src="src"
      @play="handleOnPlay"
      @pause="handleOnPause"
      @timeupdate="handleOnTime"
      @loadedmetadata="handleOnLoaded"
      @ended="handleOnEnded"
    />
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";

const props = defineProps<{
  src?: string;
  title?: string;
  artist?: string;
  artwork?: string;
}>();

const audio = useTemplateRef("audio");
const isPlaying = ref(false);
const duration = ref(0);
const currentTime = ref(0);
const seekPos = ref(0);

function formattedTime(t: number) {
  if (!t || isNaN(t)) return "0:00";
  const mm = Math.floor(t / 60);
  const ss = Math.floor(t % 60)
    .toString()
    .padStart(2, "0");
  return `${mm}:${ss}`;
}

function togglePlay() {
  if (!audio.value) return;
  if (audio.value.paused) {
    audio.value.play();
  } else {
    audio.value.pause();
  }
}

function handlePrevious() {}
function handleNext() {}

function handleOnSeekInput() {
  currentTime.value = seekPos.value;
}

function handleOnSeekChange() {
  if (!audio.value) return;
  audio.value.currentTime = seekPos.value;
}

function handleOnPlay() {
  isPlaying.value = true;
  emit("play");
}

function handleOnPause() {
  isPlaying.value = false;
  emit("pause");
}

function handleOnTime() {
  currentTime.value = audio.value?.currentTime || 0;
  seekPos.value = currentTime.value;
  emit("timeupdate", currentTime.value);
}

function handleOnLoaded() {
  duration.value = audio.value?.duration || 0;
}

function handleOnEnded() {
  isPlaying.value = false;
  emit("ended");
}
</script>
