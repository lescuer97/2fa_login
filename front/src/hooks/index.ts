import { browser } from '$app/env';

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
	if (!browser && import.meta.env.VITE_ENVIROMENT === 'develop') {
		process.env['NODE_TLS_REJECT_UNAUTHORIZED'] = 0;
	}

	const url = `${import.meta.env.VITE_AUTH_SERVER}/auth/checklogin`;
	const res = await fetch(url, {
		headers: event.request.headers
	}).catch((err) => console.log({ err }));

	if (res.status === 422 && !event.url.pathname.startsWith('/login')) {
		return Response.redirect(`${import.meta.env.VITE_AUTH_SERVER}/login`, 303);
	}
	if (res.status === 200 && event.url.pathname.startsWith('/login')) {
		return Response.redirect(import.meta.env.VITE_AUTH_SERVER, 303);
	}

	const response = await resolve(event);
	return response;
}
