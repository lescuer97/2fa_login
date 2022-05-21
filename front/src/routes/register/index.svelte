<script lang="ts">
	import { goto } from '$app/navigation';
	import { object, string, ref } from 'yup';
	import yupParser from '$lib/utils/YupSchemaParser';
	import FormInput from '$lib/FormInput.svelte';

	let email: string = '';
	let password: string = '';
	let password_repeat: string = '';
	let errorForm: any = {};

	let registerSchema = object().shape({
		email: string().email('Invalid Email Format').required('Required field'),
		password: string()
			.matches(
				/^\S*(?=\S{8,})(?=\S*\d)(?=\S*[A-Z])(?=\S*[a-z])(?=\S*[!@#$%^&*? ])\S*$/,
				'Min 8 characters, at least 1 upper case, 1 lower, one number and one special character'
			)
			.required('Required field'),
		password_repeat: string()
			.oneOf([ref('password'), null], "Passwords don't match!")
			.required('Required')
	});
	const registerUser = async () => {
		const values = await yupParser(registerSchema, { email, password, password_repeat }).catch(
			(err) => {
				errorForm = err;
				return null;
			}
		);

		if (!values) {
			return;
		}

		const req = await fetch(`${import.meta.env.VITE_AUTH_SERVER}/auth/register`, {
			method: 'post',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email,
				password,
				password_repeat
			})
		});

		if (req.ok) {
			goto('/login');
		}
	};
</script>

<div class="wrapper">
	<h3>Please Register</h3>
	<form class="register" on:submit|preventDefault={registerUser}>
		<FormInput bind:value={email} type="text" placeholder="email" error={errorForm?.email} />
		<FormInput
			bind:value={password}
			type="password"
			placeholder="password"
			error={errorForm?.password}
		/>
		<FormInput
			bind:value={password_repeat}
			type="password"
			placeholder="password repeat"
			error={errorForm?.password_repeat}
		/>

		<button type="submit">Register</button>
	</form>
</div>

<style lang="scss">
	.wrapper {
		padding: 6rem;
		background-color: #fff;
		border-radius: 5px;
		text-align: center;
		box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
	}
	.register {
		display: flex;
		flex-direction: column;
		row-gap: 10px;
	}
	button {
		background: rgb(144, 239, 110);
		padding: 12px;
		border-radius: 3px;
		font-size: 14px;
		border: 1px solid #8a8a8a;
		cursor: pointer;
	}
</style>
