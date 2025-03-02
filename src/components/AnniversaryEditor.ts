import { ref, watch, computed } from 'vue';
import { createAnniversary, updateAnniversary } from '../utils/apiClient';
import type { Ref } from 'vue';

export interface AnniversaryEditorProps {
  anniversaryToEdit: { id: number; name: string; date: string } | null;
}

export interface AnniversaryEditorEmits {
  (e: 'anniversarySaved'): void;
  (e: 'cancelEdit'): void;
}

export function useAnniversaryEditor(
  anniversaryToEdit: Ref<AnniversaryEditorProps['anniversaryToEdit']>,
  emit: AnniversaryEditorEmits
) {
  const newAnniversaryName = ref(anniversaryToEdit.value?.name || "");
  const newAnniversaryDate = ref(anniversaryToEdit.value?.date || "");
  const formMessage = ref("");
  const isSubmitting = ref(false);

  const isEditing = computed(() => !!anniversaryToEdit.value);

  watch(() => anniversaryToEdit.value, (newValue) => {
    if (newValue) {
      newAnniversaryName.value = newValue.name || "";
      newAnniversaryDate.value = newValue.date || "";
    } else {
      clearForm();
    }
    formMessage.value = "";
  }, { immediate: true });

  async function saveAnniversary() {
    // Validate inputs
    if (!newAnniversaryName.value.trim()) {
      formMessage.value = "请输入纪念日名称";
      return;
    }
    
    if (!newAnniversaryDate.value) {
      formMessage.value = "请选择日期";
      return;
    }

    // Validate date format (YYYY-MM-DD)
    if (!/^\d{4}-\d{2}-\d{2}$/.test(newAnniversaryDate.value)) {
      formMessage.value = "日期格式不正确";
      return;
    }

    formMessage.value = "";
    isSubmitting.value = true;

    try {
      let success = false;
      
      if (isEditing.value) {
        // Ensure we have a valid ID before updating
        if (!anniversaryToEdit.value || typeof anniversaryToEdit.value.id !== 'number') {
          throw new Error("无效的纪念日ID");
        }
        
        success = await updateAnniversary(
          anniversaryToEdit.value.id,
          newAnniversaryName.value.trim(),
          newAnniversaryDate.value
        );
      } else {
        success = await createAnniversary(
          newAnniversaryName.value.trim(),
          newAnniversaryDate.value
        );
      }

      if (success) {
        formMessage.value = isEditing.value ? "纪念日已更新!" : "新纪念日已创建!";
        if (!isEditing.value) clearForm();
        emit('anniversarySaved');
      } else {
        formMessage.value = "操作失败，请稍后再试";
      }
    } catch (error: any) {
      console.error("Failed to save anniversary:", error);
      formMessage.value = error.message || "保存失败，请稍后再试";
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

  return {
    newAnniversaryName,
    newAnniversaryDate,
    formMessage,
    isSubmitting,
    isEditing,
    saveAnniversary,
    clearForm,
    handleCancel
  };
}