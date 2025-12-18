export class BbeanError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'BbeanError';
    }
}

export class ConnectionError extends BbeanError {
    constructor(message: string) {
        super(message);
        this.name = 'ConnectionError';
    }
}

export class TaskError extends BbeanError {
    constructor(message: string) {
        super(message);
        this.name = 'TaskError';
    }
}

export class ProofError extends BbeanError {
    constructor(message: string) {
        super(message);
        this.name = 'ProofError';
    }
}

export class TimeoutError extends BbeanError {
    constructor(message: string) {
        super(`timeout: ${message}`);
        this.name = 'TimeoutError';
    }
}
