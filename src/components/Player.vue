<script setup>
import {ref, watch} from 'vue';

const props = defineProps({
  // the video this player is currently playing
  src: String,
  // the type of video this player is currently playing, e.g. 'video/mp4'
  type: String,
});

const emit = defineEmits(['timeupdate']);

const videoEl = ref(null);

// watch the props.src variable
// if it changed, we manually reload the video
watch(() => props.src, (newVal, oldVal) => {
  console.log('src has been changed to', newVal);
  // do nothing if the video element doesn't exist yet (may happen on first startup)
  if (videoEl.value === null) return;

  // call #load on the video element
  console.log('Calling #load on:', videoEl.value);
  videoEl.value.load();
  console.log('Success', videoEl.value);
});

function seekTo(time) {
  videoEl.value.currentTime = time;
}

function play() {
  videoEl.value.play();
}

function pause() {
  videoEl.value.pause();
}

// expose the #seekTo method to parents
defineExpose({seekTo, play, pause});
</script>

<template>
  <video controls
         class="bg-gray-800 min-h-0 min-w-0"
         ref="videoEl"
         @timeupdate="(e) => emit('timeupdate', e.target.currentTime, e.target.paused)">
    <source :src="props.src" :type="props.type">
  </video>
</template>
