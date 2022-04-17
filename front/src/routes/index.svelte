<script context="module">
	import { browser } from '$app/env';

	if (!browser) {
		process.env['NODE_TLS_REJECT_UNAUTHORIZED'] = 0;
	}
	export async function load({ params, fetch, session, stuff }) {
		const url = `${import.meta.env.VITE_AUTH_SERVER}/auth/checklogin`;
		const res = await fetch(url);

		if (res.status === 307) {
			return { status: res.status, redirect: '/login' };
		}
		return {
			props: 'ok'
		};
	}
</script>

<script lang="ts">
	import axios from 'axios';

	const logout = async () => {
		const res = await axios
			.post(`${import.meta.env.VITE_AUTH_SERVER}/auth/logout`)
			.then((res) => res);

		if (res) {
			window.location = '/login';
		}
	};
</script>

<button on:click={logout}>Logout</button>
<div class="mainPage">Here a video should go</div>

<style lang="scss">
</style>
