// CE hours lookup by profession and state
export const CE_REQUIREMENTS: Record<string, Record<string, number>> = {
	'Attorney': {
		'CA': 25, 'NY': 24, 'TX': 15, 'FL': 33, 'IL': 30,
		'default': 24
	},
	'Engineer': {
		'CA': 30, 'TX': 15, 'NY': 36, 'FL': 18, 'IL': 30,
		'default': 24
	},
	'CPA': {
		'CA': 80, 'TX': 120, 'NY': 40, 'FL': 80, 'IL': 120,
		'default': 40
	},
	'Architect': {
		'CA': 30, 'TX': 12, 'NY': 36, 'FL': 20, 'IL': 24,
		'default': 24
	}
};

// AI-suggested topics by profession
export const TOPIC_SUGGESTIONS: Record<string, string[]> = {
	'Attorney': [
		'Contract Law',
		'Ethics & Professional Responsibility',
		'Litigation Strategy',
		'Legal Technology',
		'Employment Law',
		'Intellectual Property',
		'Tax Law',
		'Real Estate Law'
	],
	'Engineer': [
		'Project Management',
		'Sustainable Design',
		'Building Codes',
		'Safety Regulations',
		'Advanced Materials',
		'AutoCAD & BIM',
		'Structural Analysis',
		'Ethics'
	],
	'CPA': [
		'Tax Planning',
		'Audit Standards',
		'Financial Reporting',
		'Ethics',
		'Forensic Accounting',
		'Technology & Data Analytics',
		'Business Valuation',
		'GAAP Updates'
	],
	'Architect': [
		'Building Codes',
		'Sustainable Design',
		'BIM & Technology',
		'Project Management',
		'Historic Preservation',
		'Accessibility Standards',
		'Construction Administration',
		'Ethics'
	]
};
