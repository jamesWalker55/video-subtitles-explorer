<script setup>
const props = defineProps({
  width: Number,
});
</script>

<template>
  <!-- `v-bind="$attrs"` indicates attributes on the component should be put there -->
  <div id="sidebar" :style="{width: `${width}px`}" v-bind="$attrs">
    <slot/>
  </div>
  <div id="grabber"
       class="w-1 flex-grow-0 flex-shrink-0 bg-gray-900 hover:bg-gray-400 cursor-ew-resize">
  </div>
</template>
