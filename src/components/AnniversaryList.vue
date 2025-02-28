<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getAnniversaries, deleteAnniversary } from '../utils/apiClient';
import type { Anniversary } from '../types';
import '../styles.css';

const anniversaries = ref<Anniversary[]>([]);
const isDeleting = ref(false);
const errorMessage = ref('');
const debugInfo = ref('');

async function loadAnniversaries() {
  debugInfo.value = 'Loading anniversaries...';
  try {
    const result = await getAnniversaries();
    console.log('Loaded anniversaries:', result);
    anniversaries.value = result;
    errorMessage.value = '';
    debugInfo.value = `Loaded ${result.length} anniversaries`;
  } catch (err) {
    errorMessage.value = '加载纪念日失败';
    debugInfo.value = `Error: ${JSON.stringify(err)}`;
    console.error('Error loading anniversaries:', err);
  }
}

// ...existing code...
function getDayClass(days: number): string {
  return days < 0 ? 'past-date' : 'future-date';
}

async function handleDelete(id: number) {
  debugInfo.value = `Delete clicked for ID: ${id}`;
  console.log("Delete button clicked for ID:", id);
  if (confirm('确定要删除这个纪念日吗？')) {
    isDeleting.value = true;
    errorMessage.value = '';
    
    try {
      console.log("Calling deleteAnniversary with ID:", id);
      debugInfo.value = `Calling deleteAnniversary with ID: ${id}`;
      const success = await deleteAnniversary(id);
      console.log("Delete result:", success);
      debugInfo.value = `Delete result: ${success}`;
      
      if (success) {
        // Reload the list after successful deletion
        await loadAnniversaries();
        debugInfo.value = 'Successfully deleted and reloaded';
      } else {
        errorMessage.value = '删除纪念日失败';
        debugInfo.value = 'Delete operation returned false';
      }
    } catch (err) {
      errorMessage.value = '删除纪念日时出现错误';
      debugInfo.value = `Delete error: ${JSON.stringify(err)}`;
      console.error('Error deleting anniversary:', err);
    } finally {
      isDeleting.value = false;
    }
  } else {
    debugInfo.value = 'Delete cancelled by user';
  }
}

onMounted(() => {
  loadAnniversaries();
});

// Expose the loadAnniversaries method for the parent to call
defineExpose({ loadAnniversaries });
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
            {{ anniversary.days < 0 ? '已过去' : '还有' }} {{ Math.abs(anniversary.days) }} 天
          </p>
          <!-- Debug info to show the ID -->
          <small class="debug-id">ID: {{ anniversary.id }}</small>
        </div>
        <button 
          class="delete-btn" 
          @click="handleDelete(anniversary.id)"
          :disabled="isDeleting"
        >
          删除
        </button>
      </li>
    </ul>
  </div>
</template>

<style scoped>
/* Styles have been moved to external CSS file */
</style>