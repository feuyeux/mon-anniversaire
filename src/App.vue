<script setup lang="ts">
import { ref } from "vue";
import './styles.css';
import TabsContainer from './components/TabsContainer.vue';
import CreateAnniversary from './components/CreateAnniversary.vue';
import AnniversaryList from './components/AnniversaryList.vue';
import DateCalculator from './components/DateCalculator.vue';

const anniversaryListRef = ref<InstanceType<typeof AnniversaryList> | null>(null);

function handleAnniversaryCreated() {
  // Reload the anniversary list when a new one is created
  anniversaryListRef.value?.loadAnniversaries();
}

const tabs = [
  { id: 'create', label: '创建纪念日' },
  { id: 'list', label: '我的纪念日' },
  { id: 'calculate', label: '日期计算' }
];
</script>

<template>
  <main class="container">
    <h1>Mon Anniversaire</h1>
    
    <TabsContainer :tabs="tabs">
      <template #create>
        <CreateAnniversary @anniversary-created="handleAnniversaryCreated" />
      </template>
      
      <template #list>
        <AnniversaryList ref="anniversaryListRef" />
      </template>
      
      <template #calculate>
        <DateCalculator />
      </template>
    </TabsContainer>
  </main>
</template>