import type { Course, CompletedCourse } from '$lib/types';

export const RECOMMENDED_COURSES: Course[] = [
	{
		id: 1,
		title: 'Advanced Contract Drafting in the Digital Age',
		provider: 'State Bar of California',
		hours: 3.0,
		topic: 'Contract Law',
		format: 'Online',
		price: 'Free',
		rating: 4.8,
		aiReason: 'Matches your interest in Contract Law and Legal Technology'
	},
	{
		id: 2,
		title: 'AI and Ethics: Navigating New Challenges',
		provider: 'ABA Center for Professional Development',
		hours: 2.5,
		topic: 'Ethics & Professional Responsibility',
		format: 'Live Webinar',
		price: '$75',
		rating: 4.9,
		aiReason: 'Combines Ethics with cutting-edge technology topics'
	},
	{
		id: 3,
		title: 'Legal Tech Tools for Modern Practice',
		provider: 'Legal Innovation Institute',
		hours: 4.0,
		topic: 'Legal Technology',
		format: 'Self-Paced',
		price: '$125',
		rating: 4.7,
		aiReason: 'Perfect for enhancing your Legal Technology skills'
	},
	{
		id: 4,
		title: 'Contract Law Updates 2025',
		provider: 'State Bar of California',
		hours: 1.5,
		topic: 'Contract Law',
		format: 'Online',
		price: 'Free',
		rating: 4.6,
		aiReason: 'Stay current with latest Contract Law changes'
	}
];

export const RECENT_COURSES: CompletedCourse[] = [
	{
		id: 101,
		title: 'Professional Responsibility Essentials',
		completedDate: '2025-01-15',
		hours: 3.0,
		status: 'Certified'
	},
	{
		id: 102,
		title: 'Business Law Fundamentals',
		completedDate: '2024-12-10',
		hours: 2.5,
		status: 'Certified'
	}
];
