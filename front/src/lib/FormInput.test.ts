import FormInput from './FormInput.svelte';

import { it, expect, describe } from 'vitest';
import { render } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

describe('Login Component', () => {
	it('Rendered component', () => {
		const value = '';

		const { getByPlaceholderText } = render(FormInput, {
			type: 'text',
			placeholder: 'test placeholder',
			value
		});

		const element = getByPlaceholderText('test placeholder');

		expect(element).toBeTruthy();
	});
	it('Input values', async () => {
		const value = '';
		const { getByPlaceholderText } = render(FormInput, {
			type: 'text',
			placeholder: 'test placeholder',
			value
		});
		const user = userEvent.setup();

		const element = getByPlaceholderText('test placeholder');
		await user.click(element);
		await user.keyboard('User for testing yeyyy');

		// the value variable doesn't get set because is not inside svelte
		expect(element.value).toBe('User for testing yeyyy');
	});
	it('preset value on input', async () => {
		const value = 'preset test value';
		const { getByPlaceholderText } = render(FormInput, {
			type: 'text',
			placeholder: 'test placeholder',
			value
		});
		// const user = userEvent.setup();

		const element = getByPlaceholderText('test placeholder');

		expect(element.value).toBe('preset test value');
	});

	it('should have email type input', async () => {
		const value = 'preset test value';
		const { getByPlaceholderText } = render(FormInput, {
			type: 'email',
			placeholder: 'test placeholder',
			value
		});
		// const user = userEvent.setup();

		const element = getByPlaceholderText('test placeholder');

		expect(element.type).toBe('email');
	});

	it('error field from outside prop', async () => {
		const value = 'preset test value';
		const { getByText } = render(FormInput, {
			type: 'text',
			placeholder: 'test placeholder',
			value,
			error: 'this is a test error message'
		});
		// const user = userEvent.setup();

		const element = getByText('this is a test error message');

		expect(element).toBeTruthy();
	});
});
