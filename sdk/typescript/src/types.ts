export enum TaskPriority {
    Low = 'low',
    Normal = 'normal',
    High = 'high',
    Critical = 'critical',
}

export enum TaskState {
    Queued = 'queued',
    Assigned = 'assigned',
    Running = 'running',
    Completed = 'completed',
    Failed = 'failed',
}

export interface Task {
    id: string;
    modelId: string;
    payload: Uint8Array;
    priority: TaskPriority;
    callbackUrl?: string;
    createdAt: Date;
}

export interface TaskReceipt {
    id: string;
    status: TaskStatus;
    queuedAt: Date;
    estimatedWaitSecs?: number;
}

export interface TaskStatus {
    state: TaskState;
    nodeId?: string;
    startedAt?: Date;
    result?: TaskResult;
    error?: string;
    retries?: number;
}

export interface TaskResult {
    output: Uint8Array;
    proofHash: string;
    computeTimeMs: number;
    nodeId: string;
}

export interface NodeInfo {
    id: string;
    address: string;
    capabilities: NodeCapabilities;
    status: string;
    connectedAt: Date;
    lastHeartbeat: Date;
}

export interface NodeCapabilities {
    webgpu: boolean;
    maxModelSizeMb: number;
    supportedFormats: string[];
    computeScore: number;
}

export interface ProofData {
    taskId: string;
    nodeId: string;
    inputHash: string;
    outputHash: string;
    nonce: number;
    difficulty: number;
    timestamp: Date;
}

export interface EngineStatus {
    running: boolean;
    version: string;
    nodeCount: number;
    queueLength: number;
    totalTasksProcessed: number;
}

export interface RewardInfo {
    pendingRewards: number;
    totalClaimed: number;
    lastClaimAt?: Date;
}
