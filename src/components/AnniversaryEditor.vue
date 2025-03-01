<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { createAnniversary, updateAnniversary } from '../utils/apiClient';
import '../styles.css';

const props = defineProps<{
  anniversaryToEdit: { id: number; name: string; date: string } | null;
}>();

const newAnniversaryName = ref(props.anniversaryToEdit?.name || "");
const newAnniversaryDate = ref(props.anniversaryToEdit?.date || "");
const formMessage = ref("");
const isSubmitting = ref(false);

// Computed property to check if we're in edit mode
const isEditing = computed(() => !!props.anniversaryToEdit);

const emit = defineEmits(['anniversarySaved', 'cancelEdit']);

watch(() => props.anniversaryToEdit, (newValue) => {
  newAnniversaryName.value = newValue?.name || "";
  newAnniversaryDate.value = newValue?.date || "";
  formMessage.value = "";
}, { immediate: true });

async function saveAnniversary() {
  if (!newAnniversaryName.value || !newAnniversaryDate.value) {
    formMessage.value = "请填写所有必填字段";
    return;
  }

  formMessage.value = "";
  isSubmitting.value = true;

  try {
    let success = false;

    if (isEditing.value) {
      // Edit existing anniversary
      success = await updateAnniversary(
        props.anniversaryToEdit!.id,
        newAnniversaryName.value,
        newAnniversaryDate.value
      );
    } else {
      // Create new anniversary
      success = await createAnniversary(
        newAnniversaryName.value,
        newAnniversaryDate.value
      );
    }

    if (success) {
      formMessage.value = isEditing.value ? "纪念日已更新!" : "新纪念日已创建!";
      // Clear form after successful create (but not after edit)
      if (!isEditing.value) {
        clearForm();
      }

      // Emit event to parent to reload the list
      emit('anniversarySaved');
    } else {
      formMessage.value = "操作失败，请稍后再试";
    }
  } catch (error) {
    console.error("Failed to save anniversary:", error);
    formMessage.value = "保存失败，请稍后再试";
  } finally {
    isSubmitting.value = false;
  }
}

function clearForm() {
  newAnniversaryName.value = "";
  newAnniversaryDate.value = "";
  formMessage.value = "";
}

function handleCancel() {
  if (isEditing.value) {
    emit('cancelEdit');
  } else {
    clearForm();
  }
}
</script>

<template>
  <div class="card">
    <h2>{{ isEditing ? '编辑纪念日' : '创建纪念日' }}</h2>

    <div v-if="formMessage" class="form-message" :class="{ success: formMessage.includes('已') }">
      {{ formMessage }}
    </div>

    <form class="form" @submit.prevent="saveAnniversary">
      <div class="form-group">
        <label for="name">纪念日名称:</label>
        <input id="name" v-model="newAnniversaryName" placeholder="例如: 生日、结婚纪念日..." required />
      </div>
      <div class="form-group">
        <label for="date">日期:</label>
        <input id="date" v-model="newAnniversaryDate" type="date" required />
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