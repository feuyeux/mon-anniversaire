<script setup lang="ts">
import { ref } from 'vue';
import '../styles.css';

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
</script>

<template>
  <div class="date-calculator">
    <h2>日期计算</h2>

    <div class="calculation-type-selector">
      <label v-for="type in calculationTypes" :key="type.id" class="radio-label">
        <input type="radio" :value="type.id" v-model="selectedType" name="calculationType">
        {{ type.label }}
      </label>
    </div>

    <div v-if="selectedType === 'dateDiff'" class="calculation-panel">
      <h3>计算两个日期之间的天数</h3>
      <div class="form-group">
        <label>开始日期:</label>
        <input type="date" v-model="startDate" class="form-control">
      </div>

      <div class="form-group">
        <label>结束日期:</label>
        <input type="date" v-model="endDate" class="form-control">
      </div>

      <button @click="calculateDateDifference" class="btn btn-primary">计算</button>

      <div v-if="dateDiffResult" class="result">
        {{ dateDiffResult }}
      </div>
    </div>

    <div v-if="selectedType === 'dateAdd'" class="calculation-panel">
      <h3>日期加减计算</h3>
      <div class="form-group">
        <label>基准日期:</label>
        <input type="date" v-model="baseDate" class="form-control">
      </div>

      <div class="form-group">
        <label>
          <input type="radio" value="add" v-model="addOrSubtract" name="operation">
          增加
        </label>
        <label>
          <input type="radio" value="subtract" v-model="addOrSubtract" name="operation">
          减少
        </label>
      </div>

      <div class="form-group">
        <label>天数:</label>
        <input type="number" v-model.number="daysToAddOrSubtract" min="0" class="form-control">
      </div>

      <button @click="calculateDateAddSubtract" class="btn btn-primary">计算</button>

      <div v-if="dateAddResult" class="result">
        {{ dateAddResult }}
      </div>
    </div>
  </div>
</template>