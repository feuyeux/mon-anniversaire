<script setup lang="ts">
import { ref, computed } from "vue";
import './styles.css';
import TabsContainer from './components/TabsContainer.vue';
import AnniversaryList from './components/AnniversaryList.vue';
import DateCalculator from './components/DateCalculator.vue';
import AnniversaryEditor from './components/AnniversaryEditor.vue';
import type { Anniversary } from './types';

const anniversaryListRef = ref<InstanceType<typeof AnniversaryList> | null>(null);
const anniversaryToEdit = ref<Anniversary | null>(null);
const isDirectTabSwitch = ref(true);

// Compute dynamic tab labels based on edit state
const tabConfig = computed(() => [
  { id: 'list', label: '纪念日' },
  { id: 'create', label: anniversaryToEdit.value ? '纪念日*' : '纪念日+' },
  { id: 'calculate', label: '日期计算' }
]);

const activeTabId = ref('list');

function handleAnniversarySaved() {
  // Reload the anniversary list when a new one is created or edited
  anniversaryListRef.value?.loadAnniversaries();

  // Clear the edit state
  anniversaryToEdit.value = null;

  // Switch back to the list tab
  selectTab('list');
}

function handleCancelEdit() {
  // Clear the anniversary being edited
  anniversaryToEdit.value = null;

  // Switch back to the list tab
  selectTab('list');
}

function handleEditRequest(anniversary: Anniversary) {
  // Set the anniversary to edit
  anniversaryToEdit.value = { ...anniversary };

  // Set flag to indicate this isn't a direct tab click
  isDirectTabSwitch.value = false;

  // Switch to the create/edit tab
  selectTab('create');

  // Reset flag after tab switch
  isDirectTabSwitch.value = true;
}

function handleAddNew() {
  // Clear any existing edit state
  anniversaryToEdit.value = null;

  // Set flag to indicate this isn't a direct tab click
  isDirectTabSwitch.value = false;

  // Switch to the create tab
  selectTab('create');

  // Reset flag after tab switch
  isDirectTabSwitch.value = true;
}

function selectTab(tabId: string) {
  // Reset edit state if moving away from create tab
  if (activeTabId.value === 'create' && tabId !== 'create') {
    anniversaryToEdit.value = null;
  }

  // If directly clicking on the create tab (not via edit button),
  // reset to create mode
  if (tabId === 'create' && isDirectTabSwitch.value) {
    anniversaryToEdit.value = null;
  }

  activeTabId.value = tabId;
}
</script>

<template>
  <main class="container">
    <h1>Mon Anniversaire</h1>

    <TabsContainer :tabs="tabConfig" :activeTabId="activeTabId" @update-active-tab="selectTab">
      <template #list>
        <div class="list-container">
          <AnniversaryList ref="anniversaryListRef" @edit-anniversary="handleEditRequest" />
          <button class="create-btn" @click="handleAddNew">添加新纪念日</button>
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

<style scoped></style>