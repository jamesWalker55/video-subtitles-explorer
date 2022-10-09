<script setup>
import {computed} from 'vue';

const props = defineProps({
  // the list of cues to be rendered
  cue: Object,
  // the type of video this player is currently playing, e.g. 'video/mp4'
  isCurrent: Boolean,
});

const emit = defineEmits(['seek']);

const textParts = computed(() => {
  // this regex: (?<=_____)
  // it is a positive lookahead, ensures the match begins with the thing it but doesn't include it in the match
  // this regex: (?<=[\.\?])[ $]
  // it means (a space or end-of-line that starts with a "." or "?")
  return props.cue.text.split(/(?<=[\.\?])[ $]/);
});

// return whether the text ends as a sentence, i.e. ends with a '.' or a '?'
const endsWithTerminator = computed(() => {
  const trimmedText = props.cue.text.trim();
  return trimmedText[trimmedText.length - 1].match(/^[\.\?]$/);
});

function formatDuration(duration) {
  const totalSecs = duration.secs + duration.nanos * 1e-9;

  const hours = Math.floor(totalSecs / 60 / 60);
  const minutes = Math.floor((totalSecs - hours * 60 * 60) / 60);
  const seconds = Math.round(totalSecs - hours * 60 * 60 - minutes * 60);

  if (hours === 0) {
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  } else {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  }
}
</script>

<template>
  <span class="group inline after:content-['_']"
      :class="props.isCurrent ? 'bg-amber-300 hover:bg-amber-400 active:bg-amber-500' : 'hover:bg-gray-100 active:bg-gray-300'"
      @click="emit('seek', cue.start)">
    <template v-for="(text, index) in textParts">
      {{text}}
      <!-- a paragraph break, put this at the end of '.' and '?' -->
      <br class="block content-[''] mt-3" v-if="index !== textParts.length - 1 || endsWithTerminator" />
    </template>
  </span>
</template>
