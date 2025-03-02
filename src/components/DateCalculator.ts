import { ref } from 'vue';

export function useDateCalculator() {
  // Date calculation types
  const calculationTypes = [
    { id: 'dateDiff', label: '两个日期之间的天数' },
    { id: 'dateAdd', label: '日期加减计算' }
  ];

  const selectedType = ref(calculationTypes[0].id);

  // For date difference calculation
  const startDate = ref('');
  const endDate = ref('');
  const dateDiffResult = ref('');

  // For date addition/subtraction
  const baseDate = ref('');
  const daysToAddOrSubtract = ref(0);
  const addOrSubtract = ref('add'); // 'add' or 'subtract'
  const dateAddResult = ref('');

  function calculateDateDifference() {
    if (!startDate.value || !endDate.value) {
      dateDiffResult.value = '请选择开始和结束日期';
      return;
    }

    const start = new Date(startDate.value);
    const end = new Date(endDate.value);
    const diffTime = Math.abs(end.getTime() - start.getTime());
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    dateDiffResult.value = `相差 ${diffDays} 天`;
  }

  function calculateDateAddSubtract() {
    if (!baseDate.value) {
      dateAddResult.value = '请选择基准日期';
      return;
    }

    const date = new Date(baseDate.value);
    const days = addOrSubtract.value === 'add'
      ? daysToAddOrSubtract.value
      : -daysToAddOrSubtract.value;

    date.setDate(date.getDate() + days);

    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');

    dateAddResult.value = `计算结果: ${year}-${month}-${day}`;
  }

  return {
    calculationTypes,
    selectedType,
    startDate,
    endDate,
    dateDiffResult,
    baseDate,
    daysToAddOrSubtract,
    addOrSubtract,
    dateAddResult,
    calculateDateDifference,
    calculateDateAddSubtract
  };
}