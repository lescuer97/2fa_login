import Login from './index.svelte';
import { it, expect, describe, afterEach } from 'vitest';
import { render, cleanup, act } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

describe('Login Component', () => {
	afterEach(async () => {
		cleanup();
	});
	it('Rendered component', () => {
		const { getByText } = render(Login);
		const element = getByText('Please Login');

		expect(element).toBeTruthy();
	});

	it('Input email and password', async () => {
		render(Login);
		const user = userEvent.setup();
		const email = document.getElementById('email').firstElementChild;
		const password = document.getElementById('password').firstElementChild;

		await user.click(email);
		await user.keyboard('test@test.com');

		await user.click(password);
		await user.keyboard('&#8V*n%!WL5^544#Z7xr');

		expect(email.value).toBe('test@test.com');
		expect(password.value).toBe('&#8V*n%!WL5^544#Z7xr');
	});

	it('Input email and password and submit the value', async () => {
		const { container } = render(Login);

		const user = userEvent.setup();
		const email = document.getElementById('email').firstElementChild;
		const password = document.getElementById('password').firstElementChild;
		const button = container.getElementsByTagName('button');

		await user.click(email);
		await user.keyboard('test@test.com');

		await user.click(password);
		await user.keyboard('&#8V*n%!WL5^544#Z7xr');

		await act(async () => {
			await user.click(button[0]);
		});

		expect(email.value).toBe('test@test.com');
		expect(password.value).toBe('&#8V*n%!WL5^544#Z7xr');
	});

	it('Error in input for email and password', async () => {
		const { container, getAllByText } = render(Login);
		const user = userEvent.setup();

		// const email = document.getElementById('email').firstElementChild;
		// const password = document.getElementById('password').firstElementChild;
		const button = container.getElementsByTagName('button');

		await act(async () => {
			await user.click(button[0]);
		});

		const errorElement = getAllByText('Required field');

		expect(errorElement[0]).toBeTruthy();
	});
});
