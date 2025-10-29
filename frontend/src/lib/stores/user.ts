import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface StateHours {
	state: string;
	hoursCompleted: number;
	renewalDate: string;
}

export interface UserData {
	username: string;
	fullName: string;
	stateHours: StateHours[];
}

// Initialize with default/empty values
const defaultUser: UserData = {
	username: '',
	fullName: '',
	stateHours: []
};

let persistedUser = browser && localStorage.getItem('user')
export let user = writable(persistedUser ? JSON.parse(persistedUser) : '')

if (browser) {
    user.subscribe(u => localStorage.user = u)
}

// Helper functions
export function updateUser(data: Partial<UserData>) {
	user.update(current => ({ ...current, ...data }));
}

export function resetUser() {
	user.set(defaultUser);
}
