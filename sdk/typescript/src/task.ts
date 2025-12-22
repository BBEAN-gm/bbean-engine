import { BbeanClient } from './client';
import { Task, TaskPriority, TaskReceipt, TaskStatus } from './types';

export class TaskBuilder {
    private client: BbeanClient;
    private modelId: string;
    private payload: Uint8Array = new Uint8Array(0);
    private priority: TaskPriority = TaskPriority.Normal;
    private callbackUrl?: string;

    constructor(client: BbeanClient, modelId: string) {
        this.client = client;
        this.modelId = modelId;
    }

    withPayload(data: Uint8Array | string): TaskBuilder {
        if (typeof data === 'string') {
            this.payload = new TextEncoder().encode(data);
        } else {
            this.payload = data;
        }
        return this;
    }

    withPriority(priority: TaskPriority): TaskBuilder {
        this.priority = priority;
        return this;
    }

    withCallback(url: string): TaskBuilder {
        this.callbackUrl = url;
        return this;
    }

    build(): Task {
        return {
            id: crypto.randomUUID(),
            modelId: this.modelId,
            payload: this.payload,
            priority: this.priority,
            callbackUrl: this.callbackUrl,
            createdAt: new Date(),
        };
    }

    async submit(): Promise<TaskReceipt> {
        const task = this.build();
        return this.client.submitTask(task);
    }

    async submitAndWait(pollIntervalMs?: number): Promise<TaskStatus> {
        const receipt = await this.submit();
        return this.client.waitForCompletion(receipt.id, pollIntervalMs);
    }
}
