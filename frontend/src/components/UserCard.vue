<script lang="ts" setup>
import type { User } from '@/types';
import { type Reactive, reactive} from 'vue';
interface Props {
    data: User,
};
const props = defineProps<Props>();

</script>

<template>
  <section
    class="bg-amber-50 p-4 border border-amber-200 text-amber-900 rounded-lg shadow-sm transition duration-300 hover:-translate-y-1 hover:shadow-lg"
    
  >
    <p 
        v-for="(value, key) in data" 
        :key="key"
        class=" px-2 overflow-hidden py-1 rounded hover:bg-amber-100 transition ease-in-out hover:-translate-0.5 ">
        {{ key }}: {{ value }}
    </p>
  </section>
</template>