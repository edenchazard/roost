<template>
  <div class="flex items-center gap-2 py-1">
    <Icon :name="icon" />
    <span>{{ label }}</span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  icon: string;
  label: string;
}>();
</script>
