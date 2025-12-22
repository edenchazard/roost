import type { Roost } from "~/types/roost";

function createUsePlayerStore() {
  const current = ref<Roost.Track | null>(null);
  const audioElementRefId = "global-audio-element";
  const audioElement = ref<HTMLAudioElement>();

  const isPlaying = ref(false);
  const duration = ref(0);
  const currentTime = ref(0);
  const seekPos = ref(0);

  const queue = ref<Roost.Track[]>([]);

  function setAudioElement(element: HTMLAudioElement) {
    audioElement.value = element;
  }

  function togglePlay() {
    console.log("Toggling play", audioElement.value);

    if (!audioElement.value) return;

    if (audioElement.value.paused) {
      audioElement.value.play();
    } else {
      audioElement.value.pause();
    }
  }

  function handleOnSeekInput() {
    currentTime.value = seekPos.value;
  }

  function handleOnSeekChange() {
    if (!audioElement.value) return;
    audioElement.value.currentTime = seekPos.value;
  }

  function handleOnPlay() {
    isPlaying.value = true;
  }

  function handleOnPause() {
    isPlaying.value = false;
  }

  function handleOnTime() {
    currentTime.value = audioElement.value?.currentTime ?? 0;
    seekPos.value = currentTime.value;
  }

  function handleOnLoaded() {
    duration.value = audioElement.value?.duration ?? 0;
  }

  function handleOnEnded() {
    isPlaying.value = false;
  }

  return {
    current,
    audioElement,
    isPlaying,
    duration,
    currentTime,
    seekPos,
    queue,
    audioElementRefId,

    togglePlay,
    handleOnSeekInput,
    handleOnSeekChange,
    handleOnPlay,
    handleOnPause,
    handleOnTime,
    handleOnLoaded,
    handleOnEnded,
    setAudioElement,
  };
}

const usePlayerStore = createUsePlayerStore();

export default usePlayerStore;
