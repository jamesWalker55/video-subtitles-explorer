<script setup>
import CueItem from './CueItem.vue';

const props = defineProps({
  // the list of cues to be rendered
  cues: Array,
  // the index of the cue we are at right now
  currentIndex: Number,
});
</script>

<template>
  <ul id="cue-display" class="overflow-auto h-full">
    <template v-for="(cue, index) in props.cues">
      <CueItem :cue="cue" :is-current="index === props.currentIndex"/>
    </template>
  </ul>
</template>
