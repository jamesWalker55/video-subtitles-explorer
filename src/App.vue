<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {convertFileSrc} from '@tauri-apps/api/tauri';
import {open} from '@tauri-apps/api/dialog';
import Player from './components/Player.vue';
import CueDisplay from './components/CueDisplay.vue';
import {ref} from 'vue';
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
</script>

<template>
  <div id="overlay"></div>
  <div
      id="root-container"
      class="flex items-stretch flex-col sm:flex-row">

    <div class="basis-1/2">
      <Player ref="player" :src="playerSrc" type="video/mp4"/>
    </div>

    <div class="basis-1/2">
      <CueDisplay :cues="cues" :current-index="7"/>
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
