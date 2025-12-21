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

    async getTaskStatus(taskId: string): Promise<TaskStatus> {
        this.ensureConnected();
        return this.request<TaskStatus>('GET', `/tasks/${taskId}/status`);
    }

    async waitForCompletion(taskId: string, pollIntervalMs: number = 1000): Promise<TaskStatus> {
        this.ensureConnected();
        const terminalStates = new Set([TaskState.Completed, TaskState.Failed]);

        for (let i = 0; i < this.timeout / pollIntervalMs; i++) {
            const status = await this.getTaskStatus(taskId);
            if (terminalStates.has(status.state)) {
                return status;
            }
            await sleep(pollIntervalMs);
        }
        throw new TaskError(`task ${taskId} did not complete within timeout`);
    }

    async getNodes(): Promise<NodeInfo[]> {
        this.ensureConnected();
        return this.request<NodeInfo[]>('GET', '/nodes');
    }

    async getEngineStatus(): Promise<EngineStatus> {
        return this.request<EngineStatus>('GET', '/status');
    }

    task(modelId: string): TaskBuilder {
        return new TaskBuilder(this, modelId);
    }

    private ensureConnected(): void {
        if (!this.connected) {
            throw new ConnectionError('not connected to engine');
        }
    }

    private async request<T>(method: string, path: string, body?: unknown): Promise<T> {
        const url = `${this.endpoint}${path}`;
        const headers: Record<string, string> = {
            'Content-Type': 'application/json',
        };
        if (this.apiKey) {
            headers['Authorization'] = `Bearer ${this.apiKey}`;
        }

        let lastError: Error | null = null;
        for (let attempt = 0; attempt < this.maxRetries; attempt++) {
            try {
                const controller = new AbortController();
                const timeoutId = setTimeout(() => controller.abort(), this.timeout);

                const response = await fetch(url, {
                    method,
                    headers,
                    body: body ? JSON.stringify(body) : undefined,
                    signal: controller.signal,
                });

                clearTimeout(timeoutId);

                if (!response.ok) {
                    const errorText = await response.text();
                    throw new BbeanError(`HTTP ${response.status}: ${errorText}`);
                }

                return (await response.json()) as T;
            } catch (error) {
                lastError = error as Error;
                if (attempt < this.maxRetries - 1) {
                    await sleep(Math.pow(2, attempt) * 1000);
                }
            }
        }
        throw lastError ?? new BbeanError('request failed');
    }
}

function sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
