export type Login = {
  email: string | undefined;
  password: string | undefined;
};

export type RegisterData = {
  email: string | undefined;
  password: string;
  password_repeat: string;
};
