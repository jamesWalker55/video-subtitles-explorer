<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {convertFileSrc} from '@tauri-apps/api/tauri';
import {open} from '@tauri-apps/api/dialog';
import Player from './components/Player.vue';
import CueDisplay from './components/CueDisplay.vue';
import {computed, ref} from 'vue';
import {invoke} from '@tauri-apps/api';

const player = ref(null);
const playerSrc = ref('');
const cues = ref([]);

async function buttonClick() {
  console.log('Clicked!');
  const path = await open({
    filters: [
      {name: 'Video', extensions: ['mp4', 'mkv']},
    ],
  });
  playerSrc.value = convertFileSrc(path);
  const vttPath = await invoke('locate_vtt', {videoPath: path});
  console.log('vttPath', vttPath);
  cues.value = await invoke('read_vtt', {path: vttPath});
  console.log('cues', cues.value);
  // player.value.load();
  // player.value.seekTo(100);
  // player.value.play();
}

function getCurrentCueIndex(cues, currentTime) {
  for (let i = 0; i < cues.length; i++) {
    const cue = cues[i];
    const start = cue.start.secs + cue.start.nanos * 1e-9;
    const end = cue.end.secs + cue.end.nanos * 1e-9;
    if (start <= currentTime && currentTime < end) {
      return i;
    }
  }
  return -1;
}

const playbackTime = ref(-1);

const currentCueIndex = computed(() => {
  return getCurrentCueIndex(cues.value, playbackTime.value);
});
</script>

<template>
  <div id="overlay"></div>
  <div
      id="root-container"
      class="flex items-stretch flex-col sm:flex-row relative">

    <button type="button"
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded absolute top-5 left-5 z-10"
            @click="buttonClick">
      Select file...
    </button>

    <div class="basis-1/2">
      <Player ref="player" :src="playerSrc" type="video/mp4" @timeupdate="(secs, isPaused) => playbackTime = secs"/>
    </div>

    <div id="grabber"
         class="w-1 bg-gray-800 hover:bg-gray-400 cursor-ew-resize">
    </div>

    <div class="basis-1/2 h-full">
      <CueDisplay :cues="cues" :current-index="currentCueIndex"/>
    </div>
  </div>
</template>

<style scoped>
#root-container {
  height: 100%;
  width: 100%;
//background: red;
}
</style>
