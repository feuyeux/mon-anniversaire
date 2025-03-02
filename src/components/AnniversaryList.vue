<template>
  <div class="card anniversary-container">
    <h2>我的纪念日</h2>

    <div v-if="errorMessage" class="error-message">
      {{ errorMessage }}
    </div>

    <div v-if="isDeleting" class="loading-overlay">
      <span>删除中...</span>
    </div>

    <div class="anniversary-content">
      <div v-if="anniversaries.length === 0" class="no-data">
        还没有纪念日，请添加一个！
      </div>
      <ul v-else class="anniversary-list">
        <li v-for="anniversary in anniversaries" :key="anniversary.id" class="anniversary-item">
          <div class="anniversary-info">
            <h3>{{ anniversary.name }}</h3>
            <div class="date-information">
              <span class="anniversary-date">{{ anniversary.date }}</span>
              <span :class="['anniversary-countdown', getDayClass(anniversary.days)]">
                {{ anniversary.days < 0 ? '已过去' : '还有' }} {{ Math.abs(anniversary.days) }} 天 </span>
            </div>
          </div>
          <div class="anniversary-actions">
            <button class="edit-btn" @click="editAnniversary(anniversary)">编辑</button>
            <button class="delete-btn" @click="handleDelete(anniversary.id)" :disabled="isDeleting">
              删除
            </button>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useAnniversaryList } from './AnniversaryList';
import '../styles.css';
import type { Anniversary } from '../types';

const emit = defineEmits(['edit-anniversary']);

const handleEditAnniversary = (anniversary: Anniversary) => {
  emit('edit-anniversary', anniversary);
};

const {
  anniversaries,
  errorMessage,
  isDeleting,
  loadAnniversariesList,
  handleDelete,
  editAnniversary,
  getDayClass
} = useAnniversaryList(handleEditAnniversary);

// Expose the loadAnniversaries method to parent components
defineExpose({ loadAnniversaries: loadAnniversariesList });

onMounted(loadAnniversariesList);
</script>