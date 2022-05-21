<script lang="ts">
	import { object, string } from 'yup';
	import { goto } from '$app/navigation';
	import FormInput from '$lib/FormInput.svelte';
	import yupParser from '$lib/utils/YupSchemaParser';

	let email: string = '';
	let password: string = '';
	let errorForm: any = {};

	let userSchema = object().shape({
		email: string().email('Invalid Email Format').required('Required field'),
		password: string().required('Required field')
	});

	const loginUser = async () => {
		const values = await yupParser(userSchema, { email, password }).catch((err) => {
			errorForm = err;
			return null;
		});

		if (!values) {
			return;
		}

		const rawRes = await fetch(`${import.meta.env.VITE_AUTH_SERVER}/auth/login`, {
			method: 'post',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email,
				password
			})
		})
			.then((res) => {
				console.log({ res });
				return res;
			})
			.catch((err) => {
				errorForm = { email: err?.data, password: err?.data };
				return err;
			});

		if (rawRes.ok) {
			goto('/');
		} else {
			const jsonData = await rawRes.json();
			errorForm = { email: jsonData?.data, password: jsonData?.data };
		}
	};
</script>

<div class="wrapper">
	<h3>Please Login</h3>
	<form class="login" on:submit|preventDefault={loginUser}>
		<FormInput type="text" placeholder="email" bind:value={email} error={errorForm?.email} />
		<FormInput
			bind:value={password}
			type="password"
			placeholder="password"
			error={errorForm?.password}
		/>

		<button type="submit">Login</button>
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
	.login {
		display: flex;
		flex-direction: column;
		row-gap: 10px;

		/* max-width: 25%; */
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
