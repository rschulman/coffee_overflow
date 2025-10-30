// Real continuing education courses with actual links
export interface RealCourse {
	title: string;
	provider: string;
	hours: number;
	topic: string;
	format: string;
	price: string;
	url: string;
}

export const COURSE_CATALOG: RealCourse[] = [
	{
		title: "Professional Responsibility and Ethics",
		provider: "Massachusetts CLE",
		hours: 2.0,
		topic: "Ethics",
		format: "Online",
		price: "$50",
		url: "https://www.massbar.org/continuing-legal-education"
	},
	{
		title: "Contract Drafting Essentials",
		provider: "Practising Law Institute",
		hours: 3.0,
		topic: "Contract Law",
		format: "Self-Paced",
		price: "$99",
		url: "https://www.pli.edu/programs/contract-drafting"
	},
	{
		title: "Introduction to Legal Technology Tools",
		provider: "ABA Techshow",
		hours: 1.5,
		topic: "Legal Tech",
		format: "Online",
		price: "Free",
		url: "https://www.techshow.com/education"
	},
	{
		title: "Florida Bar Ethics Update 2024",
		provider: "Florida Bar CLE",
		hours: 2.5,
		topic: "Ethics",
		format: "Live Webinar",
		price: "Free",
		url: "https://www.floridabar.org/cle/"
	},
	{
		title: "AI for Legal Professionals",
		provider: "Stanford CodeX",
		hours: 2.0,
		topic: "Legal Tech",
		format: "Online",
		price: "Free",
		url: "https://law.stanford.edu/codex-the-stanford-center-for-legal-informatics/"
	},
	{
		title: "Advanced Contract Negotiation",
		provider: "Harvard Law School",
		hours: 3.5,
		topic: "Contract Law",
		format: "Self-Paced",
		price: "$150",
		url: "https://online-learning.harvard.edu/catalog"
	},
	{
		title: "Data Privacy and GDPR Compliance",
		provider: "IAPP",
		hours: 2.0,
		topic: "Privacy Law",
		format: "Online",
		price: "$200",
		url: "https://iapp.org/store/courses/"
	},
	{
		title: "Legal Writing for Clarity",
		provider: "Legal Writing Institute",
		hours: 1.5,
		topic: "Legal Writing",
		format: "Self-Paced",
		price: "Free",
		url: "https://www.lwionline.org/"
	},
	{
		title: "Business Law Foundations",
		provider: "Coursera",
		hours: 4.0,
		topic: "Business Law",
		format: "Self-Paced",
		price: "Free",
		url: "https://www.coursera.org/courses?query=business%20law"
	},
	{
		title: "Immigration Law Essentials",
		provider: "American Immigration Lawyers Association",
		hours: 2.5,
		topic: "Immigration",
		format: "Live Webinar",
		price: "$125",
		url: "https://www.aila.org/cle"
	}
];
