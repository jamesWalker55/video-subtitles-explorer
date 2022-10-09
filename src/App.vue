<script setup>
import {invoke} from '@tauri-apps/api';
import {convertFileSrc} from '@tauri-apps/api/tauri';
import {open} from '@tauri-apps/api/dialog';
import {computed, onMounted, onUnmounted, ref} from 'vue';

import Player from './components/Player.vue';
import CueList from './components/CueDoc.vue';
import Toolbar from './components/Toolbar.vue';
import Sidebar from './components/Sidebar.vue';
import ToolbarButton from './components/ToolbarButton.vue';

import OpenIcon from '/src/assets/folder-open.svg';
import TargetIcon from '/src/assets/target.svg';

const log = console.log;

// the Player element
const player = ref(null);
// the source URL currently loaded in the player
const playerSrc = ref('');
// the current playback position of the video player
const playbackTime = ref(-1);

// the CueList element
const cueDisplay = ref(null);
// the list of cues currently loaded in the cue list
const cues = ref([]);

// current width of the sidebar
const sidebarWidth = ref(400);

const windowWidth = (function() {
  const widthRef = ref(window.innerWidth);

  function updateWidth() {
    widthRef.value = window.innerWidth;
  }

  onMounted(() => window.addEventListener('resize', updateWidth));
  onUnmounted(() => window.removeEventListener('resize', updateWidth));

  return widthRef;
})();

async function selectVideo() {
  const videoPath = await open({
    filters: [{name: 'Video', extensions: ['mp4', 'mkv']}],
  });
  if (videoPath === null) return;

  playerSrc.value = convertFileSrc(videoPath);

  try {
    const vttPath = await invoke('locate_vtt', {videoPath: videoPath});
    cues.value = await invoke('read_vtt', {path: vttPath});
  } catch (e) {
    // do nothing
  }
}

async function selectVtt() {
  const vttPath = await open({
    filters: [{name: 'Subtitles', extensions: ['vtt']}],
  });
  if (vttPath === null) return;

  try {
    cues.value = await invoke('read_vtt', {path: vttPath});
  } catch (e) {
    // do nothing
  }
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

const currentCueIndex = computed(() => {
  return getCurrentCueIndex(cues.value, playbackTime.value);
});
</script>

<template>
  <div id="overlay"></div>
  <div
      id="root-container"
      class="flex flex-row">

    <Sidebar class="flex-grow-0 flex-shrink-0 flex flex-col h-full" v-model:width="sidebarWidth" :min-width="200" :max-width="windowWidth - 300">
      <Toolbar>
        <span class="text-gray-500 tracking-wider">
          Video
          <span class="ml-2 mr-0">|</span>
        </span>
        <ToolbarButton @click="selectVideo" title="Open video file...">
          <OpenIcon/>
        </ToolbarButton>
      </Toolbar>
      <Player
          class="flex-shrink flex-grow"
          ref="player"
          :src="playerSrc"
          type="video/mp4"
          @timeupdate="(secs, isPaused) => playbackTime = secs"
      />
    </Sidebar>

    <div class="flex-grow flex-shrink flex flex-col">
      <Toolbar>
        <span class="text-gray-500 tracking-wider">
          Subtitles
          <span class="ml-2 mr-0">|</span>
        </span>
        <ToolbarButton @click="selectVtt" title="Open subtitles file...">
          <OpenIcon/>
        </ToolbarButton>
        <ToolbarButton @click="() => {cueDisplay.scrollToIndex(currentCueIndex)}" title="Scroll to current cue">
          <TargetIcon/>
        </ToolbarButton>
      </Toolbar>
      <CueList
          ref="cueDisplay"
          :cues="cues"
          :current-index="currentCueIndex"
          @seek="(time) => player.seekTo(time.secs + time.nanos * 1e-9)"
      />
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
