import { ProofData } from './types';

export class ProofVerifier {
    private difficulty: number;

    constructor(difficulty: number = 16) {
        this.difficulty = difficulty;
    }

    async verify(proof: ProofData): Promise<boolean> {
        const hash = await this.computeHash(proof);
        const leadingZeros = this.countLeadingZeroBits(hash);
        return leadingZeros >= this.difficulty;
    }

    private async computeHash(proof: ProofData): Promise<Uint8Array> {
        const encoder = new TextEncoder();
        const data = new Uint8Array([
            ...encoder.encode(proof.taskId),
            ...encoder.encode(proof.nodeId),
            ...encoder.encode(proof.inputHash),
            ...encoder.encode(proof.outputHash),
            ...new Uint8Array(new BigUint64Array([BigInt(proof.nonce)]).buffer),
        ]);
        const hashBuffer = await crypto.subtle.digest('SHA-256', data);
        return new Uint8Array(hashBuffer);
    }

    private countLeadingZeroBits(hash: Uint8Array): number {
        let count = 0;
        for (const byte of hash) {
            if (byte === 0) {
                count += 8;
            } else {
                let b = byte;
                while ((b & 0x80) === 0) {
                    count++;
                    b <<= 1;
                }
                break;
            }
        }
        return count;
    }

    getDifficulty(): number {
        return this.difficulty;
    }
}
