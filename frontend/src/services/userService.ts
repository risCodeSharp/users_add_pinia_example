import { api } from './api';
import type { User, UserPayload } from '@/types';

export const userService = {
    async getUsers(): Promise<User[]> {
        try {
            const response = await api.get<User[]>('/users');
            return response.data;
        } catch (error) {
            console.error("Failed to fetch users: ", error);
            throw error
        }
    },

    async createUser(user: UserPayload): Promise<User> {
        try {
            console.log(user);
            const response = await api.post<User>('/users', user);
            return response.data;
        } catch (error) {
            console.log("Faild to create user:", error);
            throw error;
        }
    }
}