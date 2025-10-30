import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface StateHours {
	state: string;
	hoursCompleted: number;
	hoursRequired: number;
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

// Load user from localStorage with proper error handling
function getInitialUser(): UserData {
	if (!browser) return defaultUser;

	try {
		const stored = localStorage.getItem('user');

		// Check for invalid/empty values
		if (!stored || stored === '' || stored === 'undefined' || stored === 'null') {
			return defaultUser;
		}

		// Try to parse
		const parsed = JSON.parse(stored);

		// Validate it's a proper UserData object with required fields
		if (
			typeof parsed === 'object' &&
			parsed !== null &&
			'username' in parsed &&
			'fullName' in parsed &&
			'stateHours' in parsed &&
			Array.isArray(parsed.stateHours)
		) {
			return parsed as UserData;
		}

		// Data structure doesn't match, clear it
		localStorage.removeItem('user');
		return defaultUser;
	} catch (e) {
		// JSON parse failed, clear corrupted data silently
		localStorage.removeItem('user');
		return defaultUser;
	}
}

export let user = writable<UserData>(getInitialUser());

// Save user to localStorage whenever it changes
if (browser) {
	user.subscribe(u => {
		try {
			localStorage.setItem('user', JSON.stringify(u));
		} catch (e) {
			console.error('Failed to save user data to localStorage:', e);
		}
	});
}

// Helper functions
export function updateUser(data: Partial<UserData>) {
	user.update(current => ({ ...current, ...data }));
}

export function resetUser() {
	user.set(defaultUser);
}
