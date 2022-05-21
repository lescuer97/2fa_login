import { rest } from 'msw';

// Define handlers that catch the corresponding requests and returns the mock data.
export const handlers = [
	rest.post(`${import.meta.env.VITE_AUTH_SERVER}/auth/login`, (req, res, ctx) => {
		return res(
			ctx.status(200),
			ctx.json({
				status: 'Success',
				data: 'Already Logged in'
			})
		);
	}),
	rest.post(`${import.meta.env.VITE_AUTH_SERVER}/auth/register`, (req, res, ctx) => {
		return res(
			ctx.status(200),
			ctx.json({
				status: 'Success',
				data: 'Registed successfuly'
			})
		);
	})
];
export default handlers;
