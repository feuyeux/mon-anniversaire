<script setup lang="ts">
import { ref } from 'vue';
import '../styles.css';

const props = defineProps<{
  tabs: { id: string; label: string }[];
}>();

const activeTab = ref(props.tabs[0]?.id || '');

function selectTab(tabId: string) {
  activeTab.value = tabId;
}
</script>

<template>
  <div class="tabs-container">
    <div class="tabs-header">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        @click="selectTab(tab.id)"
        :class="['tab-button', { active: activeTab === tab.id }]"
      >
        {{ tab.label }}
      </button>
    </div>
    
    <div class="tab-content">
      <div v-for="tab in tabs" :key="tab.id" v-show="activeTab === tab.id">
        <slot :name="tab.id"></slot>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tabs-container {
  width: 100%;
}

.tabs-header {
  display: flex;
  border-bottom: 1px solid #ddd;
  margin-bottom: 1rem;
}

.tab-button {
  padding: 0.75rem 1.5rem;
  background: transparent;
  border: none;
  border-bottom: 3px solid transparent;
  color: #666;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 500;
  transition: all 0.2s;
}

.tab-button:hover {
  background-color: #f5f5f5;
}

.tab-button.active {
  color: #3399ff;
  border-bottom: 3px solid #3399ff;
  background-color: #f0f8ff;
}

.tab-content {
  padding: 1rem 0;
}
</style>