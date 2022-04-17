<script context="module">
	// export async function load({ params, fetch, session, stuff }) {
	// 	const url = `${import.meta.env.VITE_AUTH_SERVER}/auth/checklogin`;
	// 	const res = await fetch(url);
	// 	const values = await res.json();

	// 	if (res.status === 307 || !values.data?.valid_token) {
	// 		return { status: 307, redirect: '/login' };
	// 	}
	// 	return {
	// 		props: {
	// 			data: values
	// 		}
	// 	};
	// }
</script>

<script>
	import '../styles/global.css';
</script>

<main class="content">
	<slot />
</main>

<style lang="scss">
	.content {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100%;
	}
</style>
