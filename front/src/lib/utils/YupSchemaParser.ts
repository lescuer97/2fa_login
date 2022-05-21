import type { ObjectShape } from 'node_modules/yup/lib/object';
import type ObjectSchema from 'node_modules/yup/lib/object';

export default async function yupParser(schema: ObjectSchema<ObjectShape>, valuesToCheck: unknown) {
	const validation = await schema
		.validate(valuesToCheck, { abortEarly: false, strict: true })
		.catch((errors) => {
			const errorMessages: object = {};

			errors.inner.map((err) => {
				errorMessages[err.path] = err.message;
			});
			throw errorMessages;
		});

	return validation;
}
