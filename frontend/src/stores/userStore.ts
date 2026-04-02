import { defineStore } from 'pinia';
import { type Ref, ref, computed } from 'vue';
import { userService } from '@/services/userService';
import type { User, UserPayload } from '@/types';

export const useUserStore = defineStore('user', () => {
    const users: Ref<User[]> = ref<User[]>([]);
    const loading: Ref<boolean>  = ref(false);
    const err: Ref<string | null> = ref<string | null>(null);

    const userCount = computed(() => users.value.length);

    const fetchUsers = async () => {
        loading.value = true;
        err.value = null;

        try {
            users.value = await userService.getUsers()
        } catch (error: any) {
            err.value = error.message || 'Failed to fetch users';
        } finally {
            loading.value = false
        }
    }

    const addUser = async (newUser: UserPayload) => {
        loading.value = true;
        err.value = null;

        try {
            const createdUser: User = await userService.createUser(newUser);
            users.value.push(createdUser);
        } catch (error: any) {
            err.value = error.message || 'Failed to create user';
        } finally {
            loading.value = false
        }
    }

    return {
        users,
        loading,
        err,
        userCount,
        fetchUsers,
        addUser
    }
});