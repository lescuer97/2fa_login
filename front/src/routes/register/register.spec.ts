import Register from './index.svelte';
import { it, expect, describe } from 'vitest';
import { render, act } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

describe('Register Component', () => {
	it('Rendered component', () => {
		const { getByText } = render(Register);
		const element = getByText('Please Register');

		expect(element).toBeTruthy();
	});

	it('Input email, password and password_repeat', async () => {
		render(Register);
		const user = userEvent.setup();
		const email = document.getElementById('email').firstElementChild;
		const password = document.getElementById('password').firstElementChild;
		const password_repeat = document.getElementById('password repeat').firstElementChild;

		await user.click(email);
		await user.keyboard('test@test.com');

		await user.click(password);
		await user.keyboard('&#8V*n%!WL5^544#Z7xr');

		await user.click(password_repeat);
		await user.keyboard('&#8V*n%!WL5^544#Z7xr');

		expect(email.value).toBe('test@test.com');
		expect(password.value).toBe('&#8V*n%!WL5^544#Z7xr');
		expect(password_repeat.value).toBe('&#8V*n%!WL5^544#Z7xr');
	});

	it('Input email and password and submit the value', async () => {
		const { container } = render(Register);

		const user = userEvent.setup();
		const email = document.getElementById('email').firstElementChild;
		const password = document.getElementById('password').firstElementChild;
		const password_repeat = document.getElementById('password repeat').firstElementChild;

		const button = container.getElementsByTagName('button');

		await user.click(email);
		await user.keyboard('test@test.com');

		await user.click(password);
		await user.keyboard('&#8V*n%!WL5^544#Z7xr');

		await user.click(password_repeat);
		await user.keyboard('&#8V*n%!WL5^544#Z7xr');

		await act(async () => {
			await user.click(button[0]);
		});

		expect(email.value).toBe('test@test.com');
		expect(password.value).toBe('&#8V*n%!WL5^544#Z7xr');
	});

	it('Required field error', async () => {
		const { container, getAllByText } = render(Register);
		const user = userEvent.setup();

		const button = container.getElementsByTagName('button');

		await act(async () => {
			await user.click(button[0]);
		});

		const errorElement = getAllByText('Required field');
		expect(errorElement[0]).toBeTruthy();
	});

	it('Password dont match test', async () => {
		const { container, getAllByText } = render(Register);
		const user = userEvent.setup();

		const password = document.getElementById('password').firstElementChild;
		const password_repeat = document.getElementById('password repeat').firstElementChild;

		const button = container.getElementsByTagName('button');

		await user.click(password);
		await user.keyboard('&#8V*n%!WL5^544#Z7xr');

		await user.click(password_repeat);
		await user.keyboard('&#8V*n%!WL5^544');

		await act(async () => {
			await user.click(button[0]);
		});

		const errorElement = getAllByText("Passwords don't match!");
		expect(errorElement[0]).toBeTruthy();
	});
});
