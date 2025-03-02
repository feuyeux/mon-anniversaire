<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import './styles.css';
import TabsContainer from './components/TabsContainer.vue';
import AnniversaryList from './components/AnniversaryList.vue';
import DateCalculator from './components/DateCalculator.vue';
import AnniversaryEditor from './components/AnniversaryEditor.vue';
import type { Anniversary } from './types/index';
import { platform } from '@tauri-apps/plugin-os';

const anniversaryListRef = ref<InstanceType<typeof AnniversaryList> | null>(null);
const anniversaryToEdit = ref<Anniversary | null>(null);
const isDirectTabSwitch = ref(true);
const activeTabId = ref('list');
const osInfo = ref('');

// Get OS information on component mount
onMounted(async () => {
  try {
    const currentPlatform = platform();
    console.log(currentPlatform);
    osInfo.value = `${currentPlatform}`;
  } catch (error) {
    console.error('Failed to get OS info:', error);
    osInfo.value = 'Unknown OS';
  }
});

// Compute dynamic tab labels based on edit state
const tabConfig = computed(() => [
  { id: 'list', label: '纪念日' },
  { id: 'create', label: anniversaryToEdit.value ? '纪念日*' : '纪念日+' },
  { id: 'calculate', label: '日期计算' }
]);

function handleAnniversarySaved() {
  anniversaryListRef.value?.loadAnniversaries();
  anniversaryToEdit.value = null;
  selectTab('list');
}

function handleCancelEdit() {
  anniversaryToEdit.value = null;
  selectTab('list');
}

function handleEditRequest(anniversary: Anniversary) {
  anniversaryToEdit.value = { ...anniversary };
  isDirectTabSwitch.value = false;
  selectTab('create');
  isDirectTabSwitch.value = true;
}

function selectTab(tabId: string) {
  if (activeTabId.value === 'create' && tabId !== 'create') {
    anniversaryToEdit.value = null;
  }

  if (tabId === 'create' && isDirectTabSwitch.value) {
    anniversaryToEdit.value = null;
  }

  activeTabId.value = tabId;
}
</script>

<template>
  <main class="container">
    <div class="header-container">
      <h1>Mon Anniversaire</h1>
      <span v-if="osInfo" class="os-info">{{ osInfo }}</span>
    </div>

    <TabsContainer :tabs="tabConfig" :activeTabId="activeTabId" @update-active-tab="selectTab">
      <template #list>
        <div class="list-container">
          <AnniversaryList ref="anniversaryListRef" @edit-anniversary="handleEditRequest" />
        </div>
      </template>

      <template #create>
        <AnniversaryEditor :anniversaryToEdit="anniversaryToEdit" @anniversarySaved="handleAnniversarySaved"
          @cancelEdit="handleCancelEdit" />
      </template>

      <template #calculate>
        <DateCalculator />
      </template>
    </TabsContainer>
  </main>
</template>