<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {convertFileSrc} from '@tauri-apps/api/tauri';
import {open} from '@tauri-apps/api/dialog';
import Player from './components/Player.vue';
import {ref} from 'vue';
import {invoke} from '@tauri-apps/api';

const player = ref(null);
const playerSrc = ref("");

async function buttonClick() {
  console.log('Clicked!');
  const path = await open({
    filters: [
      {name: 'Video', extensions: ['mp4', 'mkv']},
    ],
  });
  playerSrc.value = convertFileSrc(path);
  const vttPath = await invoke('locate_vtt', {videoPath: path});
  console.log("vttPath", vttPath);
  const cues = await invoke('read_vtt', {path: vttPath});
  console.log("cues", cues);
  // player.value.load();
  // player.value.seekTo(100);
  // player.value.play();
}
</script>

<template>
  <div class="container">
    <button type="button" @click="buttonClick">Greet</button>

    <div class="row">
      <Player ref="player" :src="playerSrc" type="video/mp4"/>
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
