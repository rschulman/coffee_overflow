export interface Course {
	id: number;
	title: string;
	provider: string;
	hours: number;
	topic: string;
	format: string;
	price: string;
	url: string;
	rating?: number;
	aiReason: string;
}

export interface CompletedCourse {
	id: number;
	title: string;
	completedDate: string;
	hours: number;
	status: string;
}
