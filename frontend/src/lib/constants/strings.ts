// App-wide string constants

export const APP_NAME = 'EduTrack Pro';

export const STRINGS = {
	// Common
	appName: APP_NAME,
	appTagline: 'Your AI-powered continuing education companion',

	// Navigation
	nav: {
		dashboard: 'Dashboard',
		courses: 'Courses',
		profile: 'Profile',
		signIn: 'Sign In',
		alreadyHaveAccount: 'Already have an account?',
	},

	// Login Page
	login: {
		welcomeBack: 'Welcome Back',
		email: 'Email',
		password: 'Password',
		signIn: 'Sign In',
		dontHaveAccount: "Don't have an account?",
		getStarted: 'Get started',
		emailLabel: 'Email address',
		passwordLabel: 'Password',
	},

	// Onboarding Page
	onboarding: {
		title: 'Welcome to EduTrack Pro!',
		subtitle: "Let's personalize your continuing education experience",

		// Section 1
		section1Title: 'Basic Information',
		fullName: 'Full Name',
		profession: 'Profession',
		state: 'State/Province',
		licenseNumber: 'License Number',
		renewalDate: 'Renewal Date',
		selectProfession: 'Select profession',
		selectState: 'Select state',

		// Section 2
		section2Title: 'Continuing Education Status',
		hoursRequired: 'Hours Required',
		hoursCompleted: 'Hours Completed',
		yourProgress: 'Your Progress',
		aiAutoFilled: 'AI auto-filled based on your profession and state',

		// Section 3
		section3Title: 'Areas of Interest',
		aiRecommendations: 'AI-powered recommendations for {profession}s',
		selectTopicsPrompt: "Select topics you're interested in to get personalized course recommendations",
		dontSeeTopics: "Don't see what you're looking for?",
		addCustomTopic: 'Add custom topic',
		add: 'Add',

		// Info box
		infoText: 'Your information is used to track requirements, suggest relevant courses, and automate certification submissions to your governing body.',

		// Buttons
		startLearning: 'Start Learning',
		back: 'Back',
	},

	// Dashboard Page
	dashboard: {
		welcomeBack: 'Welcome back, {name}! 👋',
		submitCertification: 'Submit Certification',

		// Stats
		ceHoursProgress: 'CE Hours Progress',
		currentPeriod: 'Current reporting period',
		completed: 'Completed',
		required: 'Required',
		remaining: 'Remaining',

		nextRenewal: 'Next Renewal',
		licenseExpiration: 'License expiration',
		daysRemaining: '{days} days remaining',
		notSet: 'Not set',

		onTrack: 'On Track',
		completionStatus: 'Completion status',
		almostThere: '🎉 Almost there!',
		greatProgress: '✨ Great progress!',
		keepGoing: '💪 Keep going!',
		doingGreat: "You're doing great",

		// AI Recommendations
		aiRecommendations: 'AI-Powered Recommendations',
		personalizedCourses: 'Personalized courses based on your interests: {topics}',
		personalizedCoursesDefault: 'Personalized course recommendations for you',

		// Recent Activity
		recentActivity: 'Recent Activity',
		completedOn: 'Completed {date} • {hours} hours',

		// Course Card
		enrollNow: 'Enroll Now',
		hours: '{hours} hours',

		// Alerts
		certificationAlert: '🎉 Certification submission feature coming soon! This will automatically send your completed courses to the {state} governing body.',
		enrollmentAlert: 'Enrollment for "{course}" coming soon!',
	},

	// Professions
	professions: {
		attorney: 'Attorney',
		engineer: 'Professional Engineer',
		cpa: 'Certified Public Accountant',
		architect: 'Architect',
	},

	// States
	states: {
		ca: 'California',
		ny: 'New York',
		tx: 'Texas',
		fl: 'Florida',
		il: 'Illinois',
	},

	// Course status
	courseStatus: {
		certified: 'Certified',
	},

	// Aria Labels
	aria: {
		mainNav: 'Main navigation',
		signInForm: 'Sign in form',
		onboardingForm: 'Onboarding form',
		topicSelection: 'Topic selection',
		yourFullName: 'Your full name',
		selectYourProfession: 'Select your profession',
		selectYourState: 'Select your state or province',
		yourLicenseNumber: 'Your license number',
		licenseRenewalDate: 'License renewal date',
		requiredCEHours: 'Required CE hours',
		completedCEHours: 'Completed CE hours',
		addCustomTopicField: 'Add custom topic',
		ceHoursProgress: 'CE hours completion progress',
		submitCertToStateBar: 'Submit certification to state bar',
		completeOnboarding: 'Complete onboarding and go to dashboard',
		goBackToLogin: 'Go back to login',
		enrollInCourse: 'Enroll in {course}',
		topicChip: '{topic} topic',
		loginPage: 'Login page',
		onboardingPage: 'Onboarding page',
		dashboardPage: 'Dashboard',
		alreadyHaveAccountSignIn: 'Already have an account? Sign in',
		signInToAccount: 'Sign in to your account',
	},
};

// Helper function to replace placeholders in strings
export function formatString(template: string, values: Record<string, string | number>): string {
	return template.replace(/\{(\w+)\}/g, (match, key) => String(values[key] ?? match));
}
