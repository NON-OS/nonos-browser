export interface NetworkStatus {
	connected: boolean;
	bootstrap_progress: number;
	circuits: number;
	status: string;
	error: string | null;
}

export interface Circuit {
	id: string;
	path: string[];
}
