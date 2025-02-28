<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import '../styles.css';

const newAnniversaryName = ref("");
const newAnniversaryDate = ref("");

const emit = defineEmits(['anniversaryCreated']);

async function saveAnniversary() {
  if (!newAnniversaryName.value || !newAnniversaryDate.value) return;

  try {
    await invoke("save_anniversary", {
      name: newAnniversaryName.value,
      date: newAnniversaryDate.value,
    });

    newAnniversaryName.value = "";
    newAnniversaryDate.value = "";

    // Emit event to parent to reload the list
    emit('anniversaryCreated');
  } catch (error) {
    console.error("Failed to save anniversary:", error);
  }
}
</script>

<template>
  <div class="card">
    <h2>创建纪念日</h2>
    <form class="form" @submit.prevent="saveAnniversary">
      <div class="form-group">
        <label for="name">纪念日名称:</label>
        <input id="name" v-model="newAnniversaryName" placeholder="例如: 生日、结婚纪念日..." />
      </div>
      <div class="form-group">
        <label for="date">日期:</label>
        <input id="date" v-model="newAnniversaryDate" type="date" />
      </div>
      <button type="submit">保存</button>
    </form>
  </div>
</template>