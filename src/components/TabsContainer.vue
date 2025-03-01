<script setup lang="ts">
import { ref, watch } from 'vue';
import '../styles.css';

const props = defineProps<{
  tabs: { id: string; label: string }[];
  activeTabId: string;
}>();

const emit = defineEmits(['update-active-tab']);

function selectTab(tabId: string) {
  emit('update-active-tab', tabId);
}

// Watch for changes to activeTabId from parent
watch(() => props.activeTabId, (newActiveTabId) => {
  // The parent will handle the state, we don't need local state anymore
});
</script>

<template>
  <div class="tabs-container">
    <div class="tabs-header">
      <button v-for="tab in tabs" :key="tab.id" @click="selectTab(tab.id)"
        :class="['tab-button', { active: activeTabId === tab.id }]">
        {{ tab.label }}
      </button>
    </div>

    <div class="tab-content">
      <div v-for="tab in tabs" :key="tab.id" v-show="activeTabId === tab.id">
        <slot :name="tab.id"></slot>
      </div>
    </div>
  </div>
</template>