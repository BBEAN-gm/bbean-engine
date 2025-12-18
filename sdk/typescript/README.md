# @bbean/sdk

TypeScript SDK for interacting with the BBEAN compute engine.

## Installation

```bash
git clone https://github.com/BBEAN-gm/bbean-engine.git
cd bbean-engine/sdk/typescript
npm install
npm run build
```

## Usage

```typescript
import { BbeanClient, TaskPriority } from '@bbean/sdk';

const client = new BbeanClient({
  endpoint: 'http://localhost:9420',
});

await client.connect();

const result = await client
  .task('llama-7b')
  .withPayload('Hello, world!')
  .withPriority(TaskPriority.Normal)
  .submitAndWait();
```

## API Reference

### BbeanClient

- `connect()` - Connect to the engine
- `disconnect()` - Disconnect
- `submitTask(task)` - Submit an inference task
- `getTaskStatus(taskId)` - Get task status
- `waitForCompletion(taskId)` - Poll until task completes
- `getNodes()` - List connected nodes
- `task(modelId)` - Create a TaskBuilder

### TaskBuilder

- `withPayload(data)` - Set task payload
- `withPriority(priority)` - Set priority level
- `withCallback(url)` - Set callback URL
- `submit()` - Submit the task
- `submitAndWait()` - Submit and wait for result
