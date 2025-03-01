<script setup lang="ts">
import { ref, onMounted, defineEmits } from 'vue';
import { loadAnniversaries, deleteAnniversary } from '../utils/apiClient';
import type { Anniversary } from '../types';

const emit = defineEmits(['edit-anniversary']);

const anniversaries = ref<Anniversary[]>([]);
const errorMessage = ref('');
const isDeleting = ref(false);
const debugInfo = ref('');

async function loadAnniversariesList() {
  try {
    anniversaries.value = await loadAnniversaries();
  } catch (error) {
    errorMessage.value = '加载纪念日失败';
    console.error(error);
  }
}

async function handleDelete(id: number) {
  isDeleting.value = true;
  try {
    const success = await deleteAnniversary(id);
    if (success) {
      anniversaries.value = anniversaries.value.filter(a => a.id !== id);
    } else {
      errorMessage.value = '删除纪念日失败';
    }
  } catch (error) {
    errorMessage.value = '删除纪念日失败';
    console.error(error);
  } finally {
    isDeleting.value = false;
  }
}

function editAnniversary(anniversary: Anniversary) {
  emit('edit-anniversary', anniversary);
}

function getDayClass(days: number) {
  return days < 0 ? 'past-date' : 'future-date';
}

// Expose the loadAnniversaries method to parent components
defineExpose({ loadAnniversaries: loadAnniversariesList });

onMounted(loadAnniversariesList);
</script>

<template>
  <div class="card">
    <h2>我的纪念日</h2>

    <div v-if="errorMessage" class="error-message">
      {{ errorMessage }}
    </div>

    <!-- Add debug info panel for development -->
    <div v-if="debugInfo" class="debug-info">
      Debug: {{ debugInfo }}
    </div>

    <div v-if="isDeleting" class="loading-overlay">
      <span>删除中...</span>
    </div>

    <div v-if="anniversaries.length === 0" class="no-data">
      还没有纪念日，请添加一个！
    </div>

    <ul v-else class="anniversary-list">
      <li v-for="anniversary in anniversaries" :key="anniversary.id" class="anniversary-item">
        <div class="anniversary-info">
          <h3>{{ anniversary.name }}</h3>
          <p>日期: {{ anniversary.date }}</p>
          <p :class="getDayClass(anniversary.days)">
            {{ anniversary.days < 0 ? '已过去' : '还有' }} {{ Math.abs(anniversary.days) }} 天 </p>
              <!-- Debug info to show the ID -->
              <small class="debug-id">ID: {{ anniversary.id }}</small>
        </div>
        <button class="edit-btn" @click="editAnniversary(anniversary)">编辑</button>
        <button class="delete-btn" @click="handleDelete(anniversary.id)" :disabled="isDeleting">
          删除
        </button>
      </li>
    </ul>

    <!-- Removed the CreateAnniversary component that doesn't belong here -->
  </div>
</template>