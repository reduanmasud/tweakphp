<script setup lang="ts">
  import { ref, watch, computed } from 'vue'
  import { useSettingsStore } from '../stores/settings'

  // Define props
  const props = defineProps({
    modelValue: [String, Number],
    placeholder: [String],
  })

  const settingsStore = useSettingsStore()

  // Local state for the selected value
  const selectedValue = ref(props.modelValue)

  // Emit the change event to update the parent component
  const emit = defineEmits(['update:modelValue'])

  const emitChange = () => {
    emit('update:modelValue', selectedValue.value)
  }

  // Watch for changes in the prop modelValue to update the local state
  watch(
    () => props.modelValue,
    newValue => {
      selectedValue.value = newValue
    }
  )

  // Computed properties for CSS v-bind
  const backgroundColor = computed(() => settingsStore.colors.backgroundLight)
  const foregroundColor = computed(() => settingsStore.colors.foreground)
</script>

<template>
  <select
    v-model="selectedValue"
    @change="emitChange"
    class="text-sm h-7 outline outline-1 py-1 px-2 focus:!outline-primary-500 rounded-md appearance-none"
    :style="{
      backgroundColor: settingsStore.colors.backgroundLight,
      color: settingsStore.colors.foreground,
      outlineColor: settingsStore.colors.border,
    }"
    v-bind="$attrs"
  >
    <option value="" selected disabled>
      {{ placeholder }}
    </option>
    <slot></slot>
  </select>
</template>

<style scoped>
select {
  background-color: v-bind('backgroundColor') !important;
  -webkit-appearance: none;
  -moz-appearance: none;
  box-sizing: border-box;
  margin: 0;
  outline-offset: 0;
}

select:focus {
  outline-offset: 0;
}

select option {
  background-color: v-bind('backgroundColor');
  color: v-bind('foregroundColor');
}
</style>
