<script lang="ts" setup>
import UserCard from './UserCard.vue';
import BarLoading from './BarLoading.vue';
import { onMounted } from 'vue';
import { useUserStore } from '@/stores/userStore';

const userStore = useUserStore();

onMounted(() => {
    userStore.fetchUsers();
});

</script>

<template>
    <h2 class="text-center "> User ({{ userStore.userCount }})</h2>
    <div class="mt-6 mx-10">
        <BarLoading v-if="userStore.loading" class="w-full mx-auto" />
        <div class=" grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 xl:grid-cols-4 gap-4" v-else>
            <UserCard v-for="user in userStore.users" :data="user" />
        </div>
    </div>
</template>