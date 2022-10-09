<script setup>
const props = defineProps({
  // the list of cues to be rendered
  cue: Object,
  // the type of video this player is currently playing, e.g. 'video/mp4'
  isCurrent: Boolean,
});

const emit = defineEmits(['seek']);

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
  <li class="flex gap-2 items-baseline px-2 group"
      :class="props.isCurrent ? 'bg-amber-300 hover:bg-amber-400 active:bg-amber-500' : 'hover:bg-gray-100 active:bg-gray-300'"
      @click="emit('seek', cue.start)">
    <div class="select-none font-mono text-sm flex-none text-gray-300 group-hover:text-gray-500 group-active:underline"
         :class="props.isCurrent ? 'text-gray-500' : ''">
      {{ formatDuration(cue.start) }}
    </div>
    <div>
      {{ cue.text }}
    </div>
  </li>
</template>
