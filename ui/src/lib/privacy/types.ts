export interface PrivacyService {
	id: string;
	name: string;
	description: string;
	category: string;
	weight: string;
	pricePerUnit: string;
	unit: string;
	icon: string;
	active: boolean;
}

export interface ActiveSession {
	id: string;
	service: string;
	startTime: number;
	usage: string;
	cost: string;
}

export interface NetworkStats {
	connected_nodes: number;
	total_bandwidth_gb: string;
	proofs_generated: number;
	mix_operations: number;
	trackers_blocked: number;
	active_users: number;
}

export interface BootstrapNode {
	id: string;
	name: string;
	ip: string;
	flag: string;
	status: 'online' | 'offline';
}
