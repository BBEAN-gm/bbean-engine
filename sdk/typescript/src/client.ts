import { Task, TaskReceipt, TaskStatus, NodeInfo, EngineStatus, TaskState } from './types';
import { BbeanError, ConnectionError, TaskError } from './errors';
import { TaskBuilder } from './task';

export interface ClientConfig {
    endpoint: string;
    apiKey?: string;
    timeout?: number;
    maxRetries?: number;
}

const DEFAULT_TIMEOUT = 30_000;
const DEFAULT_MAX_RETRIES = 3;

export class BbeanClient {
    private endpoint: string;
    private apiKey?: string;
    private timeout: number;
    private maxRetries: number;
    private connected: boolean = false;

    constructor(config: ClientConfig) {
        this.endpoint = config.endpoint.replace(/\/+$/, '');
        this.apiKey = config.apiKey;
        this.timeout = config.timeout ?? DEFAULT_TIMEOUT;
        this.maxRetries = config.maxRetries ?? DEFAULT_MAX_RETRIES;
    }

    async connect(): Promise<void> {
        const status = await this.request<EngineStatus>('GET', '/status');
        if (!status.running) {
            throw new ConnectionError('engine is not running');
        }
        this.connected = true;
    }

    async disconnect(): Promise<void> {
        this.connected = false;
    }

    isConnected(): boolean {
        return this.connected;
    }

    async submitTask(task: Task): Promise<TaskReceipt> {
        this.ensureConnected();
        const body = {
            model_id: task.modelId,
            payload: Array.from(task.payload),
            priority: task.priority,
            callback_url: task.callbackUrl,
        };
        return this.request<TaskReceipt>('POST', '/tasks', body);
    }
