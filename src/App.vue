<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {convertFileSrc} from '@tauri-apps/api/tauri';
import {open} from '@tauri-apps/api/dialog';
import Player from './components/Player.vue';
import CueDisplay from './components/CueDisplay.vue';
import {computed, ref} from 'vue';
import {invoke} from '@tauri-apps/api';
import Toolbar from './components/Toolbar.vue';
import OpenIcon from '/src/assets/folder-open.svg';
import TargetIcon from '/src/assets/target.svg';
import ToolbarButton from './components/ToolbarButton.vue';
const log = console.log;

const player = ref(null);
const playerSrc = ref('');
const cues = ref([]);

async function buttonClick() {
  const videoPath = await open({
    filters: [
      {name: 'Video', extensions: ['mp4', 'mkv']},
    ],
  });
  if (videoPath === null) return;

  playerSrc.value = convertFileSrc(videoPath);

  const vttPath = await invoke('locate_vtt', {videoPath: videoPath});
  cues.value = await invoke('read_vtt', {path: vttPath});
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

function seekPlayer(time) {
  console.log("Seeking to", time);
  player.value.seekTo(time);
}
</script>

<template>
  <div id="overlay"></div>
  <div
      id="root-container"
      class="flex items-stretch flex-col sm:flex-row relative">

    <button type="button"
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded absolute top-14 left-5 z-10"
            @click="buttonClick">
      Select file...
    </button>

    <div class="basis-1/2 flex flex-col">
      <Toolbar>
        <ToolbarButton @click="buttonClick" title="Open video file...">
          <OpenIcon/>
        </ToolbarButton>
      </Toolbar>
      <Player ref="player" :src="playerSrc" type="video/mp4" @timeupdate="(secs, isPaused) => playbackTime = secs"/>
    </div>

    <div id="grabber"
         class="w-1 bg-gray-900 hover:bg-gray-400 cursor-ew-resize">
    </div>

    <div class="basis-1/2 flex flex-col">
      <Toolbar>
        <ToolbarButton @click="log" title="Open subtitles file...">
          <OpenIcon/>
        </ToolbarButton>
        <ToolbarButton @click="log" title="Scroll to current cue">
          <TargetIcon/>
        </ToolbarButton>
      </Toolbar>
      <CueDisplay :cues="cues" :current-index="currentCueIndex" @seek="(time) => seekPlayer(time.secs + time.nanos * 1e-9)"/>
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
