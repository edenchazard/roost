<template>
  <div class="flex items-center gap-2">
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
