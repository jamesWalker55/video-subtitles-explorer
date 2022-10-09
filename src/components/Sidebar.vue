<script setup>
import {computed, onMounted, onUnmounted, ref} from 'vue';

const props = defineProps({
  width: Number,
  minWidth: Number,
  maxWidth: Number,
});

const constrainedWidth = computed(() => {
  let width = props.width;
  if (!isNaN(props.maxWidth)) {
    width = Math.min(width, props.maxWidth);
  }
  if (!isNaN(props.minWidth)) {
    width = Math.max(width, props.minWidth);
  }
  return width;
});

const emit = defineEmits(["update:width", "beginResize", "endResize"]);

const sidebar = ref(null);
const grabber = ref(null);

const grabberCallbacks = (function() {
  let isResizing = false;

  return {
    /**
     * mousedown callback, should be on the grabber element
     * @param e {MouseEvent}
     */
    mousedown(e) {
      console.log(performance.now(), "mousedown");
      isResizing = true;
      emit('beginResize');
    },
    /**
     * mousemove callback, should be on the window element (as the mouse may move too fast and leave the grabber)
     * @param e {MouseEvent}
     */
    mousemove(e) {
      if (!isResizing) return;

      // const newPos = e.pageX - grabber.value.getBoundingClientRect().width / 2;
      const newPos = e.pageX;
      console.log(performance.now(), "mousemove", newPos);
      emit('update:width', newPos);
    },
    /**
     * mouseup callback, should be on the window element (as the mouse may move too fast and leave the grabber)
     * @param e {MouseEvent}
     */
    mouseup(e) {
      console.log(performance.now(), "mouseup");
      isResizing = false;
      emit('endResize');
    },
  }
})();

onMounted(() => {
  window.addEventListener('mousemove', grabberCallbacks.mousemove);
  window.addEventListener('mouseup', grabberCallbacks.mouseup);
});

onUnmounted(() => {
  window.removeEventListener('mousemove', grabberCallbacks.mousemove);
  window.removeEventListener('mouseup', grabberCallbacks.mouseup);
});
</script>

<template>
  <!-- `v-bind="$attrs"` indicates attributes on the component should be put there -->
  <div ref="sidebar" :style="{width: `${constrainedWidth}px`}" v-bind="$attrs">
    <slot/>
  </div>
  <div ref="grabber"
       class="w-1 flex-grow-0 flex-shrink-0 bg-gray-900 hover:bg-gray-400 cursor-ew-resize"
       @mousedown.prevent="grabberCallbacks.mousedown"
  />
  <!-- use @mousedown.prevent to avoid creating/affecting text selections on the cue display list -->
</template>
