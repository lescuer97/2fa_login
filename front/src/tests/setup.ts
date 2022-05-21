import { afterAll, afterEach, beforeAll } from 'vitest';
import { server } from './mocks/server';
import 'whatwg-fetch';
import { cleanup } from '@testing-library/svelte';

beforeAll(() => server.listen({ onUnhandledRequest: 'error' }));
afterAll(() => server.close());
afterEach(() => {
	cleanup();
	server.resetHandlers();
});
