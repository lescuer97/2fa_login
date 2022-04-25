<script lang="ts">
	import { goto } from '$app/navigation';

	const logout = async () => {
		const rawRes = await fetch(`${import.meta.env.VITE_AUTH_SERVER}/auth/logout`, {
			method: 'post',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			}
		});
		// const res = await rawRes.json();
		if (rawRes.ok) {
			goto('/login');
		}
	};
</script>

<button on:click={logout}>Logout</button>
<div class="mainPage">Here a video should go</div>

<style lang="scss">
</style>
