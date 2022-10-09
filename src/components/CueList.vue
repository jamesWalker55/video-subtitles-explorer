<script setup>
import CueListItem from './CueListItem.vue';
import {ref} from 'vue';

const props = defineProps({
  // the list of cues to be rendered
  cues: Array,
  // the index of the cue we are at right now
  currentIndex: Number,
});

const emit = defineEmits(['seek']);

const displayEl = ref(null);

function scrollToIndex(index) {
  const cueEl = displayEl.value.querySelector(`[data-index="${index}"]`);
  if (cueEl === null) return;

  const viewportHeight = displayEl.value.clientHeight;
  // .offsetTop returns the position relative to the main window for some reason
  // i'm subtracting it by the parent's .offsetTop to get the position relative to the scroll area
  const cueLocalOffsetTop = cueEl.offsetTop - displayEl.value.offsetTop;
  const cueHeight = cueEl.getBoundingClientRect().height;

  const targetYpos = cueLocalOffsetTop - (viewportHeight - cueHeight) / 2;

  displayEl.value.scrollTo({top: targetYpos});
}

defineExpose({scrollToIndex});
</script>

<template>
  <ul id="cue-display" class="overflow-auto h-full" ref="displayEl">
    <template v-for="(cue, index) in props.cues">
      <CueListItem
          :cue="cue"
          :is-current="index === props.currentIndex"
          :data-index="index"
          @seek="(...args) => emit('seek', ...args)"/>
    </template>
  </ul>
</template>
