<template>
  <div class="card">
    <h2>{{ isEditing ? '编辑纪念日' : '创建纪念日' }}</h2>

    <div v-if="formMessage" class="form-message" :class="{ success: formMessage.includes('已') }">
      {{ formMessage }}
    </div>

    <form class="form" @submit.prevent="saveAnniversary">
      <div class="form-group">
        <label for="name">纪念日名称:</label>
        <input id="name" v-model="newAnniversaryName" placeholder="例如: 生日、结婚纪念日..." class="form-input" required />
      </div>
      <div class="form-group">
        <label for="date">日期:</label>
        <input id="date" v-model="newAnniversaryDate" type="date" class="form-input date-input" required />
      </div>
      <div class="form-actions">
        <button type="submit" :disabled="isSubmitting" class="primary-button">
          {{ isSubmitting ? '保存中...' : (isEditing ? '保存更改' : '创建纪念日') }}
        </button>
        <button type="button" @click="handleCancel" class="secondary-button">
          {{ isEditing ? '取消编辑' : '清空表单' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { toRef } from 'vue';
import { useAnniversaryEditor } from './AnniversaryEditor';
import '../styles.css';

const props = defineProps<{
  anniversaryToEdit: { id: number; name: string; date: string } | null;
}>();

const emit = defineEmits(['anniversarySaved', 'cancelEdit']);
const anniversaryToEditRef = toRef(props, 'anniversaryToEdit');

const {
  newAnniversaryName,
  newAnniversaryDate,
  formMessage,
  isSubmitting,
  isEditing,
  saveAnniversary,
  handleCancel
} = useAnniversaryEditor(anniversaryToEditRef, emit);
</script>