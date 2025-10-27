import { writable } from 'svelte/store';

export interface UserData {
	fullName: string;
	profession: string;
	state: string;
	licenseNumber: string;
	renewalDate: string;
	hoursRequired: number;
	hoursCompleted: number;
	selectedTopics: string[];
}

// Initialize with default/empty values
const defaultUser: UserData = {
	fullName: '',
	profession: '',
	state: '',
	licenseNumber: '',
	renewalDate: '',
	hoursRequired: 0,
	hoursCompleted: 0,
	selectedTopics: []
};

// Create the store
export const userData = writable<UserData>(defaultUser);

// Helper functions
export function updateUser(data: Partial<UserData>) {
	userData.update(current => ({ ...current, ...data }));
}

export function resetUser() {
	userData.set(defaultUser);
}
