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
import Sidebar from './components/Sidebar.vue';
import OpenIcon from '/src/assets/folder-open.svg';
import TargetIcon from '/src/assets/target.svg';
import ToolbarButton from './components/ToolbarButton.vue';
const log = console.log;

const player = ref(null);
const playerSrc = ref('');
const cueDisplay = ref(null);
const cues = ref([]);

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
      class="flex flex-row">

    <Sidebar class="flex-grow-0 flex-shrink-0 flex flex-col h-full" width="400">
      <Toolbar>
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
        <ToolbarButton @click="selectVtt" title="Open subtitles file...">
          <OpenIcon/>
        </ToolbarButton>
        <ToolbarButton @click="() => {cueDisplay.scrollToIndex(currentCueIndex)}" title="Scroll to current cue">
          <TargetIcon/>
        </ToolbarButton>
      </Toolbar>
      <CueDisplay
          ref="cueDisplay"
          :cues="cues"
          :current-index="currentCueIndex"
          @seek="(time) => seekPlayer(time.secs + time.nanos * 1e-9)"
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
